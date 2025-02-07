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
    // List can be a list of numbers (real/int)
    List(Vec<f64>),
    ListAccess(String, Box<Expr>),
    UnaryOp(String, Box<Expr>),
}

#[derive(Debug, Clone)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone)]
pub enum ParseResult {
    Success(String),
    Error(ParseError),
}

#[derive(Debug, Clone)]
pub enum ParseError {
    SyntaxError(Position),
    UndefinedVariable(String, Position),
    InvalidAtom(Position),
    IndexOutOfRange(Position, usize),
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
            Expr::List(lst) => format!("(list[({})])", lst.len()),
            Expr::ListAccess(var, idx) => format!("({}[({})])", var, idx.to_string()),
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
        println!("DEBUG [Parser]: Starting parse_calculation");
        match self.peek() {
            Some(Token::VAR(_)) if self.tokens.get(self.pos + 1) == Some(&Token::ASSIGN) => {
                println!("DEBUG [Parser]: Found assignment expression");
                self.parse_assignment()
            }
            _ => {
                let expr = self.parse_boolean()?;
                if self.pos < self.tokens.len() {
                    match self.tokens[self.pos] {
                        Token::EQ | Token::NE | Token::GT | Token::LT | Token::GE | Token::LE => {
                            println!("DEBUG [Parser]: Found comparison operator");
                            let op = match self.consume().unwrap() {
                                Token::EQ => "==",
                                Token::NE => "!=",
                                Token::GT => ">",
                                Token::LT => "<",
                                Token::GE => ">=",
                                Token::LE => "<=",
                                _ => unreachable!(),
                            };
                            println!("DEBUG [Parser]: Comparison operator: {}", op);
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
        println!("DEBUG [Parser]: Starting parse_assignment");
        let name = match self.consume() {
            Some(Token::VAR(name)) => name,
            _ => {
                println!("DEBUG [Parser]: Error - Expected variable name in assignment");
                return Err(ParseError::SyntaxError(self.get_current_position()));
            }
        };

        self.expect(Token::ASSIGN)?;

        let expr = self.parse_expression()?;
        println!("DEBUG [Parser]: Parsed assignment expression: {} = {:?}", name, expr);

        // Validate and store the assignment
        match &expr {
            Expr::List(lst) if lst.is_empty() => {
                println!("DEBUG [Parser]: Error - Empty list in assignment");
                return Err(ParseError::SyntaxError(self.get_current_position()));
            }
            _ => {
                println!("DEBUG [Parser]: Storing variable in symbol table: {}", name);
                self.variables.insert(name.clone(), expr.clone());
            }
        };

        Ok(Expr::Assignment(name, Box::new(expr)))
    }

    fn parse_boolean(&mut self) -> Result<Expr, ParseError> {
        println!("DEBUG [Parser]: Starting parse_boolean");
        let is_negative = if let Some(Token::SUB) = self.peek() {
            println!("DEBUG [Parser]: Found negative expression");
            self.consume();
            true
        } else {
            false
        };

        let mut left = self.parse_expression()?;
        println!("DEBUG [Parser]: Parsed left side of boolean: {:?}", left);

        if is_negative {
            println!("DEBUG [Parser]: Applying negative to expression");
            left = match left {
                Expr::Variable(name) => {
                    Expr::UnaryOp("-".to_string(), Box::new(Expr::Variable(name)))
                }
                Expr::Boolean(l, op, r) => match *l {
                    Expr::Variable(name) => Expr::Boolean(
                        Box::new(Expr::UnaryOp(
                            "-".to_string(),
                            Box::new(Expr::Variable(name)),
                        )),
                        op,
                        r,
                    ),
                    _ => Expr::Boolean(l, op, r),
                },
                _ => Expr::UnaryOp("-".to_string(), Box::new(left)),
            };
        }

        Ok(left)
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
                    // Allow list access in binary operations
                    match (&result, &right) {
                        (Expr::List(_), _) | (_, Expr::List(_)) => {
                            return Err(ParseError::SyntaxError(self.get_current_position()))
                        }
                        _ => {}
                    }
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

                    // Check if the next token is LIST
                    if let Some(Token::LIST) = self.peek() {
                        return Err(ParseError::SyntaxError(self.get_current_position()));
                    }

                    let right = self.parse_factor()?;

                    // Check if we're operating directly on a list (not list access)
                    match (&left, &right) {
                        (Expr::List(_), _) | (_, Expr::List(_)) => {
                            return Err(ParseError::SyntaxError(self.get_current_position()))
                        }
                        _ => {}
                    }
                    // Check if we're operating on a list
                    match (&left, &right) {
                        (Expr::List(_), _) | (_, Expr::List(_)) => {
                            return Err(ParseError::SyntaxError(self.get_current_position()))
                        }
                        _ => {}
                    }

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
        let previous_token = self.consume();
        let current_token = self.peek().cloned();

        // Check if currnet token is negative
        let is_current_negative =
            if let Some(Token::INT(n)) | Some(Token::REAL(n)) = current_token.clone() {
                let number = n.parse::<f64>().unwrap();
                number < 0.0
            } else {
                false
            };

        // If the previous and current tokens are numbers (either int/real), and last token contains a negative sign, we assume this to be a binary operation and the last number is positive
        if matches!(previous_token, Some(Token::INT(_)) | Some(Token::REAL(_)))
            && matches!(current_token, Some(Token::INT(_)) | Some(Token::REAL(_)))
            && is_current_negative
        {
            let prev = match previous_token.unwrap() {
                Token::INT(n) => self.parse_number(n.to_string(), true)?,
                Token::REAL(n) => self.parse_number(n.to_string(), false)?,
                _ => return Err(ParseError::SyntaxError(self.get_current_position())),
            };
            let curr = match current_token.unwrap() {
                Token::INT(n) => {
                    self.parse_number(n.parse::<i64>().unwrap().abs().to_string(), true)?
                }
                Token::REAL(n) => {
                    self.parse_number(n.parse::<f64>().unwrap().abs().to_string(), false)?
                }
                _ => return Err(ParseError::SyntaxError(self.get_current_position())),
            };
            let expr = Expr::BinaryOp(Box::new(prev), Token::SUB.to_string(), Box::new(curr));

            Ok(expr)
        } else {
            if let Some(token) = current_token {
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
    }

    fn parse_variable(&mut self, name: String) -> Result<Expr, ParseError> {
        self.consume();

        // Check if we're dealing with a list index access
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

            // Validate that we're accessing a list
            match &self.variables[&name] {
                Expr::List(lst) => {
                    let index = match index_expr {
                        Expr::Int(n) => n as usize,
                        _ => return Err(ParseError::SyntaxError(self.get_current_position())),
                    };

                    if index >= lst.len() {
                        return Err(ParseError::IndexOutOfRange(
                            self.get_current_position(),
                            index,
                        ));
                    }
                }
                _ => return Err(ParseError::SyntaxError(self.get_current_position())),
            }

            // Check if this is an assignment to a list index
            if let Some(Token::ASSIGN) = self.peek() {
                self.consume(); // Consume the ASSIGN token

                // Parse the value being assigned
                let value = self.parse_expression()?;

                match value.clone() {
                    // Case 1: Assigning a number directly
                    Expr::Int(n) => {
                        if let Expr::List(ref mut lst) = self.variables.get_mut(&name).unwrap() {
                            let idx = match index_expr {
                                Expr::Int(i) => i as usize,
                                _ => {
                                    return Err(ParseError::SyntaxError(
                                        self.get_current_position(),
                                    ))
                                }
                            };
                            if idx < lst.len() {
                                lst[idx] = n as f64;
                            }
                        }
                    }
                    Expr::Float(n) => {
                        if let Expr::List(ref mut lst) = self.variables.get_mut(&name).unwrap() {
                            let idx = match index_expr {
                                Expr::Int(i) => i as usize,
                                _ => {
                                    return Err(ParseError::SyntaxError(
                                        self.get_current_position(),
                                    ))
                                }
                            };
                            if idx < lst.len() {
                                lst[idx] = n;
                            }
                        }
                    }
                    // Case 2: Assigning from another list's index
                    Expr::ListAccess(other_name, other_index) => {
                        // Get the source value first
                        let source_value =
                            if let Some(Expr::List(lst)) = self.variables.get(&other_name) {
                                lst.clone()
                            } else {
                                return Err(ParseError::SyntaxError(self.get_current_position()));
                            };

                        if let Some(Expr::List(ref mut target_lst)) = self.variables.get_mut(&name)
                        {
                            let target_idx = match index_expr {
                                Expr::Int(i) => i as usize,
                                _ => {
                                    return Err(ParseError::SyntaxError(
                                        self.get_current_position(),
                                    ))
                                }
                            };
                            let source_idx = match *other_index {
                                Expr::Int(i) => i as usize,
                                _ => {
                                    return Err(ParseError::SyntaxError(
                                        self.get_current_position(),
                                    ))
                                }
                            };
                            if target_idx < target_lst.len() && source_idx < source_value.len() {
                                target_lst[target_idx] = source_value[source_idx];
                            }
                        }
                    }
                    _ => return Err(ParseError::SyntaxError(self.get_current_position())),
                }

                // Return the assignment expression
                Ok(Expr::Assignment(
                    format!("{}[({})]", name, match index_expr {
                        Expr::Int(i) => i.to_string(),
                        _ => return Err(ParseError::SyntaxError(self.get_current_position())),
                    }),
                    Box::new(value),
                ))
            } else {
                Ok(Expr::ListAccess(name, Box::new(index_expr)))
            }
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
        self.consume(); // Consume LIST token
        self.expect(Token::LBRACKET)?;

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

        let mut list = vec![0.0; size];

        if let Some(Token::ASSIGN) = self.peek() {
            self.consume(); // Consume ASSIGN token

            // Parse the value after assignment
            match self.peek() {
                // Case 1: Assigning a number
                Some(Token::INT(..)) | Some(Token::REAL(..)) => {
                    let value = match self.parse_expression()? {
                        Expr::Int(n) => n as f64,
                        Expr::Float(n) => n,
                        Expr::UnaryOp(op, expr) => {
                            let val = match *expr {
                                Expr::Int(n) => n as f64,
                                Expr::Float(n) => n,
                                _ => {
                                    return Err(ParseError::SyntaxError(
                                        self.get_current_position(),
                                    ))
                                }
                            };
                            if op == "-" {
                                -val
                            } else {
                                val
                            }
                        }
                        _ => return Err(ParseError::SyntaxError(self.get_current_position())),
                    };

                    // Fill all elements with the same value
                    list = vec![value; size];
                }

                // Case 2: Assigning another list
                Some(Token::VAR(..)) => {
                    let var_expr = self.parse_expression()?;
                    match var_expr {
                        Expr::Variable(name) => {
                            if let Some(Expr::List(source_list)) =
                                self.variables.get(&name).cloned()
                            {
                                if source_list.len() != size {
                                    return Err(ParseError::SyntaxError(
                                        self.get_current_position(),
                                    ));
                                }
                                list = source_list;
                            } else {
                                return Err(ParseError::SyntaxError(self.get_current_position()));
                            }
                        }
                        _ => return Err(ParseError::SyntaxError(self.get_current_position())),
                    }
                }

                // Case 3: Assigning another list literal
                Some(Token::LIST) => {
                    let other_list = match self.parse_expression()? {
                        Expr::List(l) => l,
                        _ => return Err(ParseError::SyntaxError(self.get_current_position())),
                    };
                    if other_list.len() != size {
                        return Err(ParseError::SyntaxError(self.get_current_position()));
                    }
                    list = other_list;
                }

                _ => return Err(ParseError::SyntaxError(self.get_current_position())),
            }
        }

        Ok(Expr::List(list))
    }

    pub fn parse_tokens(&mut self, tokens: logos::Lexer<'_, Token>) -> Vec<Result<Expr, ParseError>> {
        println!("DEBUG [Parser]: Starting to parse tokens");
        let tokens_vec = tokens.collect::<Result<Vec<_>, _>>().unwrap_or_default();
        println!("DEBUG [Parser]: Collected tokens: {:?}", tokens_vec);
        
        let mut results = Vec::new();
        let mut current_line_tokens = Vec::new();
        let mut current_line = 1;

        for token in tokens_vec.iter() {
            match token {
                Token::NEWLINE => {
                    if !current_line_tokens.is_empty() {
                        println!("DEBUG [Parser]: Processing line {}: {:?}", current_line, current_line_tokens);
                        self.tokens = current_line_tokens.clone();
                        self.pos = 0;
                        let result = self.parse();
                        println!("DEBUG [Parser]: Line {} parse result: {:?}", current_line, result);
                        
                        // Only add one error per line
                        if result.is_err() && !results.last().map_or(false, |last: &Result<Expr, ParseError>| last.is_err()) {
                            results.push(result);
                        } else if result.is_ok() {
                            results.push(result);
                        }
                    }
                    current_line_tokens.clear();
                    current_line += 1;
                }
                _ => current_line_tokens.push(token.clone()),
            }
        }

        if !current_line_tokens.is_empty() {
            println!("DEBUG [Parser]: Processing final line {}: {:?}", current_line, current_line_tokens);
            self.tokens = current_line_tokens;
            self.pos = 0;
            let result = self.parse();
            println!("DEBUG [Parser]: Final line {} parse result: {:?}", current_line, result);
            
            // Only add one error per line
            if result.is_err() && !results.last().map_or(false, |last: &Result<Expr, ParseError>| last.is_err()) {
                results.push(result);
            } else if result.is_ok() {
                results.push(result);
            }
        }

        println!("DEBUG [Parser]: All parsing complete, results: {:?}", results);
        results
    }

    pub fn parse_tokens_fancy(&mut self, input: Lexer<'_, Token>) -> Vec<String> {
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
            ParseError::IndexOutOfRange(pos, index) => {
                format!(
                    "IndexOutOfRange at line {}, pos {}, index {}",
                    pos.line, pos.column, index
                )
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
