use scanner_lib::grammar::Token;

mod parser;

fn main() {
    let input = if let Some(file_path) = std::env::args().nth(1) {
        std::fs::read_to_string(file_path).unwrap()
    } else {
        eprintln!("No input file provided.");
        std::process::exit(1);
    };

    let result = parser::parse_input(&input);
    match result {
        Ok(parsed) => println!("Parsed: {}", parsed),
        Err(err) => eprintln!("Error: {}", err),
    }
}
