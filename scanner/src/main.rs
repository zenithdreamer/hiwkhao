use logos::Logos;

mod grammar;

fn tokenize(input: &str) -> Vec<(String, grammar::Token)> {
    let mut lexer = grammar::Token::lexer(input);
    let mut tokens = Vec::new();

    while let Some(token) = lexer.next() {
        let slice = lexer.slice().to_string();
        match token {
            Ok(tok) => tokens.push((slice, tok)),
            Err(_) => tokens.push((slice, grammar::Token::ERR)),
        }
    }

    tokens
}

const DEFAULT_OUTPUT_FILE: &str = "hiwkhao.tok";

fn main() {
    let input = if let Some(file_path) = std::env::args().nth(1) {
        std::fs::read_to_string(file_path).unwrap()
    } else {
        eprintln!("No input file provided.");
        std::process::exit(1);
    };

    let lines: Vec<&str> = input.lines().collect();
    let mut final_output: Vec<String> = vec![];

    for line in lines {
        let tokens = tokenize(line);
        let formatted_output: Vec<String> = tokens
            .into_iter()
            .map(|(word, token)| {
                let token_name = match token {
                    grammar::Token::REAL => "REAL",
                    grammar::Token::INT => "INT",
                    grammar::Token::VAR => "VAR",
                    grammar::Token::ADD => "+",
                    grammar::Token::SUB => "-",
                    grammar::Token::MUL => "*",
                    grammar::Token::DIV => "/",
                    grammar::Token::POW => "POW",
                    grammar::Token::LPAREN => "LPAREN",
                    grammar::Token::RPAREN => "RPAREN",
                    grammar::Token::LBRACKET => "LBRACKET",
                    grammar::Token::RBRACKET => "RBRACKET",
                    grammar::Token::COMP => "!=",
                    grammar::Token::ASSIGN => "=",
                    grammar::Token::LIST => "list",
                    grammar::Token::ERR => "ERR",
                    _ => "UNKNOWN",
                };

                format!("{}/{}", word, token_name)
            })
            .collect();

        println!("{}", formatted_output.join(" "));
        final_output.push(formatted_output.join(" "));
    }

    let output_file = std::env::args()
        .nth(2)
        .unwrap_or(DEFAULT_OUTPUT_FILE.to_string());

    std::fs::write(output_file, final_output.join("\n")).unwrap();
}
