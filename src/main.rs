use scanner_lib;
use parser_lib::symbol_table::SymbolTable;
use parser_lib::{Parser, ParseError};
use codegen_lib;

const SCANNER_DEFAULT_OUTPUT_FILE: &str = "hiwkhao.tok";
const SYMBOL_TABLE_DEFAULT_OUTPUT_FILE: &str = "hiwkhao.csv";
const PARSER_DEFAULT_OUTPUT_FILE: &str = "hiwkhao.bracket";
const CODEGEN_DEFAULT_OUTPUT_FILE: &str = "hiwkhao.asm";

fn scanner(input: &String) {
    let result = scanner_lib::run_scanner(&input);
    //println!("{}", result.join("\n"));

    let output_file = std::env::args()
        .nth(2)
        .unwrap_or(SCANNER_DEFAULT_OUTPUT_FILE.to_string());

    std::fs::write(output_file, result.join("\n")).unwrap();
}

fn generate_code(parsed_data: Vec<Result<parser_lib::Expr, ParseError>>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    
    for parsed_expr in parsed_data {
        match parsed_expr {
            Ok(expr) => {
                let instructions = codegen_lib::generate_assembly(&expr);
                if !result.is_empty() {
                    result.push(String::new());
                }
                result.extend(instructions);
            }
            Err(err) => {
                eprintln!("Error during parsing: {:?}", err);
                if !result.is_empty() && result.last() != Some(&String::new()) {
                    result.push(String::new());
                }
                result.push("ERROR".to_string());
            }
        }
    }
    
    result.push(String::new());
    result
}

fn main() {
    let input = if let Some(file_path) = std::env::args().nth(1) {
        std::fs::read_to_string(file_path).unwrap()
    } else {
        eprintln!("No input file provided.");
        std::process::exit(1);
    };

    scanner(&input);

    let tokens = scanner_lib::tokenize(&input);
    let mut parser = Parser::new(vec![]);

    let parsed_data = parser.parse_tokens(tokens.clone());

    let result = parser.parse_tokens_fancy(tokens);

    let mut table = SymbolTable::new();
    table.process_parsed_expressions(parsed_data.clone());

    println!("{}", result.join("\n"));

    table
        .write_to_csv(SYMBOL_TABLE_DEFAULT_OUTPUT_FILE)
        .unwrap();

    let parser_output_file = std::env::args()
        .nth(2)
        .unwrap_or(PARSER_DEFAULT_OUTPUT_FILE.to_string());

    std::fs::write(parser_output_file, result.join("\n")).unwrap();

    let assembly_code = generate_code(parsed_data);
    std::fs::write(CODEGEN_DEFAULT_OUTPUT_FILE, assembly_code.join("\n")).unwrap();

    println!("Processing complete!");
    println!("Scanner output: {}", SCANNER_DEFAULT_OUTPUT_FILE);
    println!("Parser output: {}", PARSER_DEFAULT_OUTPUT_FILE);
    println!("Symbol table: {}", SYMBOL_TABLE_DEFAULT_OUTPUT_FILE);
    println!("Assembly code: {}", CODEGEN_DEFAULT_OUTPUT_FILE);
}
