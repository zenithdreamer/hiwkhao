use logos::Lexer;
use scanner_lib::grammar::Token;
use std::collections::HashMap;

pub mod symbol_table;

// Core data structures
#[derive(Debug, Clone)]
pub enum Expr {
    Int(i64),
    Float(f64),
    Variable(String),
    BinaryOp(Box<Expr>, String, Box<Expr>),
    Assignment(String, Box<Expr>),
    Boolean(Box<Expr>, String, Box<Expr>),
    List(Vec<f64>),
    ListAccess(String, Box<Expr>),
    UnaryOp(String, Box<Expr>),
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
    DivisionByZero(Position),
    MissingIndex(Position),
    TokenizeError,
}

// Improved expression string representation
impl Expr {
    fn to_string(&self) -> String {
        match self {
            Expr::Int(n) => n.to_string(),
            Expr::Float(n) => format!("{:.1}", n),
            Expr::Variable(name) => name.clone(),
            Expr::BinaryOp(left, op, right) => {
                format!("({}{}{}", left.to_string(), op, right.to_string() + ")")
            }
            Expr::UnaryOp(op, expr) => format!("({}{}", op, expr.to_string() + ")"),
            Expr::Assignment(var, expr) => format!("({}={})", var, expr.to_string()),
            Expr::Boolean(left, op, right) => {
                format!("({}{}{}", left.to_string(), op, right.to_string() + ")")
            }
            Expr::List(lst) => format!("(list[{}])", lst.len()),
            Expr::ListAccess(var, idx) => format!("({}[{}])", var, idx.to_string()),
        }
    }
}

// Token utilities
pub struct TokenInfo;

impl TokenInfo {
    pub fn token_length(token: &Token) -> usize {
        match token {
            Token::VAR(name) => name.len(),
            Token::INT(n) | Token::REAL(n) => n.len(),
            Token::EQ | Token::NE | Token::LE | Token::GE | Token::INTDIV => 2,
            Token::LIST => 4,
            Token::ADD
            | Token::SUB
            | Token::MUL
            | Token::DIV
            | Token::POW
            | Token::LPAREN
            | Token::RPAREN
            | Token::LBRACKET
            | Token::RBRACKET
            | Token::ASSIGN
            | Token::GT
            | Token::LT
            | Token::NEWLINE
            | Token::WHITESPACE
            | Token::ERR => 1,
        }
    }
}

// Parser implementation
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    variables: HashMap<String, Expr>,
    current_line: usize,
    current_column: usize,
    token_positions: Vec<usize>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            pos: 0,
            variables: HashMap::new(),
            current_line: 1,
            current_column: 1,
            token_positions: Vec::new(),
        }
    }

    // Token navigation methods
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn consume(&mut self) -> Option<Token> {
        if self.pos < self.tokens.len() {
            let token = self.tokens[self.pos].clone();
            self.pos += 1;
            self.current_column += TokenInfo::token_length(&token);

            if self.pos < self.tokens.len() {
                self.current_column += 1;
            }

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
            _ => Err(ParseError::SyntaxError(self.get_current_position())),
        }
    }

    fn get_current_position(&self) -> Position {
        Position {
            line: self.current_line,
            column: if self.pos > 0 && self.pos <= self.token_positions.len() {
                self.token_positions[self.pos - 1]
            } else if !self.token_positions.is_empty() {
                self.token_positions[0]
            } else {
                self.current_column
            },
        }
    }

    // Parsing methods
    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        self.parse_calculation()
    }

    fn parse_calculation(&mut self) -> Result<Expr, ParseError> {
        match self.peek() {
            Some(Token::VAR(_)) if self.tokens.get(self.pos + 1) == Some(&Token::ASSIGN) => {
                self.parse_assignment()
            }
            _ => {
                let expr = self.parse_boolean()?;
                if self.pos < self.tokens.len() {
                    match self.tokens[self.pos] {
                        Token::EQ | Token::NE | Token::GT | Token::LT | Token::GE | Token::LE => {
                            let op = match self.consume().unwrap() {
                                Token::EQ => "==",
                                Token::NE => "!=",
                                Token::GT => ">",
                                Token::LT => "<",
                                Token::GE => ">=",
                                Token::LE => "<=",
                                _ => unreachable!(),
                            };
                            let right = self.parse_expression()?;
                            Ok(Expr::Boolean(
                                Box::new(expr),
                                op.to_string(),
                                Box::new(right),
                            ))
                        }
                        _ => Ok(expr),
                    }
                } else {
                    Ok(expr)
                }
            }
        }
    }

    fn parse_assignment(&mut self) -> Result<Expr, ParseError> {
        let name = match self.consume() {
            Some(Token::VAR(name)) => name,
            _ => return Err(ParseError::SyntaxError(self.get_current_position())),
        };

        self.expect(Token::ASSIGN)?;

        let expr = self.parse_expression()?;

        // Validate and store the assignment
        match &expr {
            Expr::List(lst) if lst.is_empty() => {
                return Err(ParseError::SyntaxError(self.get_current_position()))
            }
            _ => self.variables.insert(name.clone(), expr.clone()),
        };

        Ok(Expr::Assignment(name, Box::new(expr)))
    }

    fn parse_boolean(&mut self) -> Result<Expr, ParseError> {
        let left = if let Some(Token::SUB) = self.peek() {
            self.consume();
            let expr = self.parse_expression()?;
            match expr {
                Expr::Variable(name) => {
                    Expr::UnaryOp("-".to_string(), Box::new(Expr::Variable(name)))
                }
                _ => expr,
            }
        } else {
            self.parse_expression()?
        };

        match self.peek() {
            Some(token) => match token {
                Token::EQ | Token::NE | Token::GT | Token::LT | Token::GE | Token::LE => {
                    let op = match self.consume().unwrap() {
                        Token::EQ => "==",
                        Token::NE => "!=",
                        Token::GT => ">",
                        Token::LT => "<",
                        Token::GE => ">=",
                        Token::LE => "<=",
                        _ => unreachable!(),
                    };
                    let right = self.parse_expression()?;
                    Ok(Expr::Boolean(
                        Box::new(left),
                        op.to_string(),
                        Box::new(right),
                    ))
                }
                _ => Ok(left),
            },
            None => Ok(left),
        }
    }

    fn parse_expression(&mut self) -> Result<Expr, ParseError> {
        let mut result = self.parse_term()?;

        while let Some(token) = self.peek() {
            match token {
                Token::ADD | Token::SUB => {
                    let op = if matches!(self.consume().unwrap(), Token::ADD) {
                        "+"
                    } else {
                        "-"
                    };
                    let right = self.parse_term()?;
                    result = Expr::BinaryOp(Box::new(result), op.to_string(), Box::new(right));
                }
                Token::EQ | Token::NE | Token::GT | Token::LT | Token::GE | Token::LE => {
                    let op = match self.consume().unwrap() {
                        Token::EQ => "==",
                        Token::NE => "!=",
                        Token::GT => ">",
                        Token::LT => "<",
                        Token::GE => ">=",
                        Token::LE => "<=",
                        _ => unreachable!(),
                    };
                    let right = self.parse_term()?;
                    result = Expr::Boolean(Box::new(result), op.to_string(), Box::new(right));
                }
                _ => break,
            }
        }
        Ok(result)
    }

    fn parse_term(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_factor()?;

        while let Some(token) = self.peek() {
            match token {
                Token::MUL | Token::DIV => {
                    let is_mul = matches!(self.consume().unwrap(), Token::MUL);
                    let right = self.parse_factor()?;

                    if !is_mul {
                        // Check for division by zero
                        match &right {
                            Expr::Float(n) if *n == 0.0 => {
                                return Err(ParseError::DivisionByZero(self.get_current_position()))
                            }
                            Expr::Int(n) if *n == 0 => {
                                return Err(ParseError::DivisionByZero(self.get_current_position()))
                            }
                            _ => {}
                        }
                    }

                    left = Expr::BinaryOp(
                        Box::new(left),
                        if is_mul { "*" } else { "/" }.to_string(),
                        Box::new(right),
                    );
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_atom()?;

        while let Some(Token::POW) = self.peek() {
            self.consume();
            let right = self.parse_factor()?;
            expr = Expr::BinaryOp(Box::new(expr), "^".to_string(), Box::new(right));
        }

        Ok(expr)
    }

    fn parse_atom(&mut self) -> Result<Expr, ParseError> {
        match self.peek().cloned() {
            Some(Token::LPAREN) => {
                self.consume();
                let expr = self.parse_expression()?;
                self.expect(Token::RPAREN)?;
                Ok(expr)
            }
            Some(Token::INT(n)) => self.parse_number(n, true),
            Some(Token::REAL(n)) => self.parse_number(n, false),
            Some(Token::VAR(name)) => self.parse_variable(name),
            Some(Token::LIST) => self.parse_list(),
            Some(Token::ERR) => Err(ParseError::SyntaxError(self.get_current_position())),
            _ => Err(ParseError::InvalidAtom(self.get_current_position())),
        }
    }

    fn parse_number(&mut self, n: String, is_int: bool) -> Result<Expr, ParseError> {
        self.consume();

        if let Some(token) = self.peek() {
            if !matches!(
                token,
                Token::ADD
                    | Token::SUB
                    | Token::MUL
                    | Token::DIV
                    | Token::INTDIV
                    | Token::POW
                    | Token::EQ
                    | Token::NE
                    | Token::GT
                    | Token::LT
                    | Token::GE
                    | Token::LE
                    | Token::LPAREN
                    | Token::RPAREN
                    | Token::RBRACKET
            ) {
                return Err(ParseError::SyntaxError(self.get_current_position()));
            }
        }

        let number = n
            .parse::<f64>()
            .map_err(|_| ParseError::SyntaxError(self.get_current_position()))?;

        if number < 0.0 {
            Ok(Expr::UnaryOp(
                "-".to_string(),
                Box::new(if is_int {
                    Expr::Int(number.abs() as i64)
                } else {
                    Expr::Float(number.abs())
                }),
            ))
        } else if is_int {
            Ok(Expr::Int(number as i64))
        } else {
            Ok(Expr::Float(number))
        }
    }

    fn parse_variable(&mut self, name: String) -> Result<Expr, ParseError> {
        self.consume();

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

    fn parse_list(&mut self) -> Result<Expr, ParseError> {
        self.consume(); // Consume 'list' token
        self.expect(Token::LBRACKET)?;

        // Check if the next token is RBRACKET (empty brackets)
        if let Some(Token::RBRACKET) = self.peek() {
            return Err(ParseError::MissingIndex(self.get_current_position()));
        }

        let size = match self.consume() {
            Some(Token::INT(n)) => n
                .parse::<usize>()
                .map_err(|_| ParseError::SyntaxError(self.get_current_position()))?,
            _ => return Err(ParseError::SyntaxError(self.get_current_position())),
        };

        if size == 0 {
            return Err(ParseError::SyntaxError(self.get_current_position()));
        }

        self.expect(Token::RBRACKET)?;
        Ok(Expr::List(vec![0.0; size]))
    }
}

// Public interface for parsing tokens
impl Parser {
    pub fn parse_tokens(&mut self, input: Lexer<'_, Token>) -> ParseResult {
        let output = self.parse_tokens_with_output(input);
        for line in output {
            println!("{}", line);
        }
        ParseResult::Success("Parsing completed".to_string())
    }

    pub fn parse_tokens_fancy(&mut self, input: Lexer<'_, Token>) -> Vec<String> {
        self.parse_tokens_with_output(input)
    }

    fn parse_tokens_with_output(&mut self, input: Lexer<'_, Token>) -> Vec<String> {
        let tokens = input.collect::<Vec<_>>();
        let lines = self.split_into_lines(tokens);
        let mut output = Vec::new();
        let mut current_line = 1;

        for (line_tokens, positions) in lines {
            self.setup_line_parsing(line_tokens, positions, current_line);

            // Print tokens for debugging
            //println!("Tokens for line {}: {:?}", current_line, self.tokens);

            match self.parse() {
                Ok(expr) => output.push(expr.to_string()),
                Err(err) => output.push(self.format_error(err)),
            }

            current_line += 1;
        }

        output
    }

    fn split_into_lines(&self, tokens: Vec<Result<Token, ()>>) -> Vec<(Vec<Token>, Vec<usize>)> {
        let mut lines = Vec::new();
        let mut current_line_tokens = Vec::new();
        let mut current_line_positions = Vec::new();
        let mut column = 1;

        for token in tokens {
            match token {
                Ok(Token::NEWLINE) => {
                    if !current_line_tokens.is_empty() {
                        lines.push((current_line_tokens, current_line_positions));
                        current_line_tokens = Vec::new();
                        current_line_positions = Vec::new();
                    }
                    column = 1;
                }
                Ok(Token::WHITESPACE) => {
                    column += 1;
                }
                Ok(tok) => {
                    current_line_positions.push(column);
                    current_line_tokens.push(tok);
                    column += TokenInfo::token_length(&current_line_tokens.last().unwrap());
                }
                Err(_) => {
                    current_line_positions.push(column);
                    current_line_tokens.push(Token::ERR);
                    column += TokenInfo::token_length(&Token::ERR);
                }
            }
        }

        if !current_line_tokens.is_empty() {
            lines.push((current_line_tokens, current_line_positions));
        }

        lines
    }

    fn setup_line_parsing(
        &mut self,
        line_tokens: Vec<Token>,
        positions: Vec<usize>,
        line_number: usize,
    ) {
        self.tokens = line_tokens;
        self.token_positions = positions;
        self.pos = 0;
        self.current_line = line_number;
        self.current_column = 1;
    }

    fn format_error(&self, err: ParseError) -> String {
        match err {
            ParseError::UndefinedVariable(var, pos) => {
                format!(
                    "Undefined variable {} at line {}, pos {}",
                    var, pos.line, pos.column
                )
            }
            ParseError::SyntaxError(pos) => {
                format!("SyntaxError at line {}, pos {}", pos.line, pos.column)
            }
            ParseError::InvalidAtom(pos) => {
                format!("Invalid atom at line {}, pos {}", pos.line, pos.column)
            }
            ParseError::IndexOutOfRange(pos) => {
                format!("IndexOutOfRange at line {}, pos {}", pos.line, pos.column)
            }
            ParseError::DivisionByZero(pos) => {
                format!("Division by zero at line {}, pos {}", pos.line, pos.column)
            }
            ParseError::MissingIndex(pos) => {
                format!(
                    "Missing index expression at line {}, pos {}",
                    pos.line, pos.column
                )
            }
            ParseError::TokenizeError => "TokenizeError".to_string(),
        }
    }
}
