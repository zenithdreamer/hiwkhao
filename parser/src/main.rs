
mod ast;
mod parser;
mod scanner;

use scanner::Token;
use chumsky::Parser;

fn main() {
    let source = vec![
        "23+8",
        "2.5 * 0",
        "5NUM^ 3.0",
        "x=5",
        "10*x",
        "x=y",
        "x!=5",
        "(2+5)",
        "x = list[2]",
        "x[0] + x[1]",
    ];

    for (line_number, line) in source.iter().enumerate() {
        let lexer = Token::lexer(line).spanned();
        let tokens: Vec<_> = lexer.collect();

        match parser::parser().parse(tokens.into_iter().map(|(token, _)| token)) {
            Ok(stmt) => println!("{:?}", stmt),
            Err(errors) => {
                for error in errors {
                    println!(
                        "SyntaxError at line {}, pos {}: {:?}",
                        line_number + 1,
                        error.span().start,
                        error
                    );
                }
            }
        }
    }
}
