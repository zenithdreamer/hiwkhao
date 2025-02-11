use parser::Parser;
use scanner;
use codegen;

const DEFAULT_OUTPUT_FILE: &str = "hiwkhao.asm";

fn main() {
    let input = if let Some(file_path) = std::env::args().nth(1) {
        std::fs::read_to_string(file_path).unwrap()
    } else {
        eprintln!("No input file provided.");
        std::process::exit(1);
    };

    // Tokenize the input
    let tokens = scanner::tokenize(&input);
    let mut parser = Parser::new(vec![]);

    // Parse each line
    let parsed_results = parser.parse_tokens(tokens);

    // Generate assembly for each parsed expression
    let mut result: Vec<String> = Vec::new();

    for parsed_expr in parsed_results {
        match parsed_expr {
            Ok(expr) => {
                let instructions = codegen::generate_assembly(&expr);
                if !result.is_empty() {
                    result.push(String::new());
                }
                result.extend(instructions);
            }
            Err(_) => {
                if !result.is_empty() && result.last() != Some(&String::new()) {
                    result.push(String::new());
                }
                result.push("ERROR".to_string());
            }
        }
    }
    
    // Add a final newline
    result.push(String::new());

    let output_file = std::env::args()
        .nth(2)
        .unwrap_or(DEFAULT_OUTPUT_FILE.to_string());

    std::fs::write(output_file, result.join("\n")).unwrap();
}