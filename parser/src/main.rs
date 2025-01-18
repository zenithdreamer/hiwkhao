use scanner_lib::grammar::Token;
use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    Variable(String),
    BinaryOp(Box<Expr>, String, Box<Expr>),
    Assignment(String, Box<Expr>),
    Boolean(Box<Expr>, String, Box<Expr>),
    List(Vec<f64>),
    ListAccess(String, Box<Expr>),
}

impl Expr {
    fn to_string(&self) -> String {
        match self {
            Expr::Number(n) => n.to_string(),
            Expr::Variable(name) => name.clone(),
            Expr::BinaryOp(left, op, right) => {
                format!("({}{}{})", left.to_string(), op, right.to_string())
            }
            Expr::Assignment(var, expr) => {
                format!("({}={})", var, expr.to_string())
            }
            Expr::Boolean(left, op, right) => {
                format!("({}{}{})", left.to_string(), op, right.to_string())
            }
            Expr::List(lst) => format!("(list[({})])", lst.len().to_string()),
            Expr::ListAccess(var, idx) => format!("({}[({})])", var, idx.to_string()),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug)]
pub enum ParseResult {
    Success(String),
    Error(ParseError),
}

#[derive(Debug)]
pub enum ParseError {
    SyntaxError(Position),
    UndefinedVariable(String, Position),
    InvalidAtom(Position),
    IndexOutOfRange(Position),
}
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    variables: HashMap<String, Expr>,
    current_line: usize,
    current_column: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            pos: 0,
            variables: HashMap::new(),
            current_line: 1,
            current_column: 1,
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn consume(&mut self) -> Option<Token> {
        if self.pos < self.tokens.len() {
            let token = self.tokens[self.pos].clone();
            self.pos += 1;
            // Update position based on the consumed token's length
            let token_length = token.to_string().len();
            self.current_column += token_length;
            Some(token)
        } else {
            None
        }
    }

    fn expect(&mut self, expected: Token) -> Result<(), ParseError> {
        match self.peek() {
            Some(token) if token == &expected => {
                self.consume();
                Ok(())
            }
            Some(_) => Err(ParseError::SyntaxError(Position {
                line: self.current_line,
                column: self.current_column,
            })),
            None => Err(ParseError::SyntaxError(Position {
                line: self.current_line,
                column: self.current_column,
            })),
        }
    }

    fn get_current_position(&self) -> Position {
        Position {
            line: self.current_line,
            column: self.current_column,
        }
    }

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        self.parse_calculation()
    }

    fn parse_calculation(&mut self) -> Result<Expr, ParseError> {
        match self.peek() {
            Some(Token::VAR(_)) => {
                let next = self.tokens.get(self.pos + 1);
                if next == Some(&Token::ASSIGN) {
                    self.parse_assignment()
                } else {
                    self.parse_boolean()
                }
            }
            _ => self.parse_boolean(),
        }
    }

    fn parse_assignment(&mut self) -> Result<Expr, ParseError> {
        if let Some(Token::VAR(name)) = self.consume() {
            self.expect(Token::ASSIGN)?;
            let expr = self.parse_expression()?;

            match &expr {
                Expr::List(lst) => {
                    if lst.is_empty() {
                        return Err(ParseError::SyntaxError(self.get_current_position()));
                    }
                    self.variables.insert(name.clone(), expr.clone());
                }
                _ => {
                    self.variables.insert(name.clone(), expr.clone());
                }
            }

            Ok(Expr::Assignment(name, Box::new(expr)))
        } else {
            Err(ParseError::SyntaxError(self.get_current_position()))
        }
    }

    fn parse_boolean(&mut self) -> Result<Expr, ParseError> {
        let left = self.parse_expression()?;

        match self.peek() {
            Some(Token::EQ) => {
                self.consume();
                let right = self.parse_expression()?;
                Ok(Expr::Boolean(
                    Box::new(left),
                    "==".to_string(),
                    Box::new(right),
                ))
            }
            Some(Token::NE) => {
                self.consume();
                let right = self.parse_expression()?;
                Ok(Expr::Boolean(
                    Box::new(left),
                    "!=".to_string(),
                    Box::new(right),
                ))
            }
            Some(Token::GT) => {
                self.consume();
                let right = self.parse_expression()?;
                Ok(Expr::Boolean(
                    Box::new(left),
                    ">".to_string(),
                    Box::new(right),
                ))
            }
            _ => Ok(left),
        }
    }

    fn parse_expression(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_term()?;

        while let Some(token) = self.peek() {
            match token {
                Token::ADD => {
                    self.consume();
                    let right = self.parse_term()?;
                    left = Expr::BinaryOp(Box::new(left), "+".to_string(), Box::new(right));
                }
                Token::SUB => {
                    self.consume();
                    let right = self.parse_term()?;
                    left = Expr::BinaryOp(Box::new(left), "-".to_string(), Box::new(right));
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_factor()?;

        while let Some(token) = self.peek() {
            match token {
                Token::MUL => {
                    self.consume();
                    let right = self.parse_factor()?;
                    left = Expr::BinaryOp(Box::new(left), "*".to_string(), Box::new(right));
                }
                Token::DIV => {
                    self.consume();
                    let right = self.parse_factor()?;
                    left = Expr::BinaryOp(Box::new(left), "/".to_string(), Box::new(right));
                }
                Token::INTDIV => {
                    self.consume();
                    let right = self.parse_factor()?;
                    left = Expr::BinaryOp(Box::new(left), "//".to_string(), Box::new(right));
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_atom()?;

        while let Some(Token::POW) = self.peek() {
            self.consume();
            let right = self.parse_factor()?;
            left = Expr::BinaryOp(Box::new(left), "^".to_string(), Box::new(right));
        }
        Ok(left)
    }

    fn parse_atom(&mut self) -> Result<Expr, ParseError> {
        match self.peek().cloned() {
            Some(Token::LPAREN) => {
                self.consume();
                let expr = self.parse_expression()?;
                self.expect(Token::RPAREN)?;
                Ok(expr)
            }
            Some(Token::INT(n)) => {
                self.consume();
                if let Some(token) = self.peek() {
                    if !matches!(
                        token,
                        Token::ADD
                            | Token::SUB
                            | Token::MUL
                            | Token::DIV
                            | Token::POW
                            | Token::EQ
                            | Token::NE
                            | Token::GT
                            | Token::LPAREN
                            | Token::RPAREN
                            | Token::RBRACKET
                    ) {
                        return Err(ParseError::SyntaxError(self.get_current_position()));
                    }
                }
                Ok(Expr::Number(n.parse::<f64>().map_err(|_| {
                    ParseError::SyntaxError(self.get_current_position())
                })?))
            }
            Some(Token::REAL(n)) => {
                self.consume();
                if let Some(token) = self.peek() {
                    if !matches!(
                        token,
                        Token::ADD
                            | Token::SUB
                            | Token::MUL
                            | Token::DIV
                            | Token::POW
                            | Token::EQ
                            | Token::NE
                            | Token::GT
                            | Token::LPAREN
                            | Token::RPAREN
                            | Token::RBRACKET
                    ) {
                        return Err(ParseError::SyntaxError(self.get_current_position()));
                    }
                }
                Ok(Expr::Number(n.parse::<f64>().map_err(|_| {
                    ParseError::SyntaxError(self.get_current_position())
                })?))
            }
            Some(Token::VAR(name)) => {
                self.consume();

                // Check if this is a list access
                if let Some(Token::LBRACKET) = self.peek() {
                    self.consume();
                    let index_expr = self.parse_expression()?;
                    self.expect(Token::RBRACKET)?;

                    if !self.variables.contains_key(&name) {
                        return Err(ParseError::UndefinedVariable(
                            name,
                            self.get_current_position(),
                        ));
                    }

                    Ok(Expr::ListAccess(name, Box::new(index_expr)))
                } else {
                    if !self.variables.contains_key(&name) {
                        return Err(ParseError::UndefinedVariable(
                            name,
                            self.get_current_position(),
                        ));
                    }
                    Ok(Expr::Variable(name))
                }
            }
            Some(Token::LIST) => {
                self.consume();
                self.expect(Token::LBRACKET)?;
                let size = if let Some(Token::INT(n)) = self.consume() {
                    n.parse::<usize>()
                        .map_err(|_| ParseError::SyntaxError(self.get_current_position()))?
                } else {
                    return Err(ParseError::SyntaxError(self.get_current_position()));
                };

                self.expect(Token::RBRACKET)?;

                if size == 0 {
                    return Err(ParseError::SyntaxError(self.get_current_position()));
                }

                Ok(Expr::List(vec![0.0; size]))
            }
            _ => Err(ParseError::InvalidAtom(self.get_current_position())),
        }
    }

    fn get_error_position(&self) -> Position {
        self.get_current_position()
    }

    pub fn parse_file(&mut self, input: &str) -> ParseResult {
        // Tokenize the entire file
        let tokens = match scanner_lib::tokenize(input).collect::<Result<Vec<_>, _>>() {
            Ok(tokens) => tokens,
            Err(_) => {
                let pos = Position {
                    line: self.current_line,
                    column: 0,
                };
                println!("SyntaxError at line {}, pos {}", pos.line, pos.column);
                return ParseResult::Error(ParseError::SyntaxError(pos));
            }
        };

        // Split tokens into lines
        let lines: Vec<Vec<Token>> = tokens
            .split(|token| match token {
                Token::NEWLINE => true,
                _ => false,
            })
            .map(|tokens| tokens.to_vec())
            .collect();

        // Process each line
        for (line_num, line_tokens) in lines.into_iter().enumerate() {
            self.tokens = line_tokens;
            self.pos = 0;
            self.current_line = line_num + 1;

            match self.parse() {
                Ok(expr) => {
                    println!("{:?}", expr);
                }
                Err(err) => {
                    println!("{:?}", err);
                }
            }
        }

        // Return success if everything parses correctly
        ParseResult::Success("Parsing completed".to_string())
    }

    pub fn parse_file_pretty(&mut self, input: &str) -> ParseResult {
        // Tokenize the entire file
        let tokens = match scanner_lib::tokenize(input).collect::<Result<Vec<_>, _>>() {
            Ok(tokens) => tokens,
            Err(_) => {
                let pos = Position {
                    line: self.current_line,
                    column: 0,
                };
                println!("SyntaxError at line {}, pos {}", pos.line, pos.column);
                return ParseResult::Error(ParseError::SyntaxError(pos));
            }
        };

        // Split tokens into lines
        let lines: Vec<Vec<Token>> = tokens
            .split(|token| match token {
                Token::NEWLINE => true,
                _ => false,
            })
            .map(|tokens| tokens.to_vec())
            .collect();

        // Process each line
        for (line_num, line_tokens) in lines.into_iter().enumerate() {
            self.tokens = line_tokens;
            self.pos = 0;
            self.current_line = line_num + 1;

            match self.parse() {
                Ok(expr) => {
                    if let Some(Token::LBRACKET) = self.peek() {
                        self.consume();
                        let idx = self.parse_expression();
                        if let Err(err) = self.expect(Token::RBRACKET) {
                            println!("{:?}", err);
                        }

                        if let Ok(Expr::Number(index)) = idx {
                            let _idx = index as usize;
                            println!("{}", expr.to_string());
                        } else {
                            println!("Invalid list access");
                        }
                    } else {
                        println!("{}", expr.to_string());
                    }
                }
                Err(err) => match err {
                    ParseError::UndefinedVariable(var, pos) => {
                        println!(
                            "Undefined variable {} at line {}, pos {}",
                            var, pos.line, pos.column
                        );
                    }
                    ParseError::SyntaxError(pos) => {
                        println!("SyntaxError at line {}, pos {}", pos.line, pos.column);
                    }
                    ParseError::IndexOutOfRange(pos) => {
                        println!("IndexOutOfRange at line {}, pos {}", pos.line, pos.column);
                    }
                    ParseError::InvalidAtom(pos) => {
                        if self.pos >= self.tokens.len() {
                            continue;
                        }
                        println!("Invalid atom at line {}, pos {}", pos.line, pos.column)
                    }
                },
            }
        }

        // Return success if everything parses correctly
        ParseResult::Success("Parsing completed".to_string())
    }
}

fn main() {
    let input = r"23+8
25 * 0
5NUM^ 3.0
x=5
10*x
x=y
x!=5
(2+5)
x = list[2]
x[0] + x[1]
";
    let mut parser = Parser::new(vec![]);
    let mut result = parser.parse_file_pretty(input);

    //println!("{:?}", result);

    //parser = Parser::new(vec![]);
    //result = parser.parse_file(input);

    //println!("{:?}", result);
}
