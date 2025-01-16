// parser/src/parser.rs
use chumsky::prelude::*;
use crate::scanner::grammar::Token;

pub fn expr() -> impl Parser<Token, Output = String> {
    let number = just(Token::INT)
        .map(|_| "number".to_string());

    let float = just(Token::FLOAT)
        .map(|_| "float".to_string());

    let parens = just(Token::LPAREN)
        .ignore_then(expr())
        .then_ignore(just(Token::RPAREN))
        .map(|inner| format!("({})", inner));

    let term = number.or(float).or(parens);

    let addition = term
        .clone()
        .then(just(Token::ADD).ignore_then(term))
        .map(|(lhs, rhs)| format!("add({},{})", lhs, rhs));

    let subtraction = term
        .clone()
        .then(just(Token::SUB).ignore_then(term))
        .map(|(lhs, rhs)| format!("sub({},{})", lhs, rhs));

    let multiplication = term
        .clone()
        .then(just(Token::MUL).ignore_then(term))
        .map(|(lhs, rhs)| format!("mul({},{})", lhs, rhs));

    let division = term
        .clone()
        .then(just(Token::DIV).ignore_then(term))
        .map(|(lhs, rhs)| format!("div({},{})", lhs, rhs));

    let power = term
        .clone()
        .then(just(Token::POW).ignore_then(term))
        .map(|(lhs, rhs)| format!("pow({},{})", lhs, rhs));

    addition
        .or(subtraction)
        .or(multiplication)
        .or(division)
        .or(power)
        .or(term)
}

pub fn parse_input(input: &str) -> Result<String, String> {
    let tokens = crate::scanner::lib::tokenize(input);
    
    let result = expr().parse(tokens.into_iter().map(|(_, t)| t));

    match result {
        Ok(parsed) => Ok(parsed),
        Err(errors) => Err(format!("Parsing failed: {:?}", errors)),
    }
}
