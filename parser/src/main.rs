use parser::symbol_table::SymbolTable;
use parser::Parser;

const SCANNER_DEFAULT_OUTPUT_FILE: &str = "hiwkhao.tok";
const SYMBOL_TABLE_DEFAULT_OUTPUT_FILE: &str = "hiwkhao.csv";
const PARSER_DEFAULT_OUTPUT_FILE: &str = "hiwkhao.bracket";

fn scanner(input: &String) {
    let result = scanner::run_scanner(&input);
    //println!("{}", result.join("\n"));

    let output_file = std::env::args()
        .nth(2)
        .unwrap_or(SCANNER_DEFAULT_OUTPUT_FILE.to_string());

    std::fs::write(output_file, result.join("\n")).unwrap();
}

fn main() {
    let input = if let Some(file_path) = std::env::args().nth(1) {
        std::fs::read_to_string(file_path).unwrap()
    } else {
        eprintln!("No input file provided.");
        std::process::exit(1);
    };

    scanner(&input);

    let tokens = scanner::tokenize(&input);
    let mut parser = Parser::new(vec![]);

    let parsed_data = parser.parse_tokens(tokens.clone());

    let result = parser.parse_tokens_fancy(tokens);

    let mut table = SymbolTable::new();
    table.process_parsed_expressions(parsed_data);

    println!("{}", result.join("\n"));

    table
        .write_to_csv(SYMBOL_TABLE_DEFAULT_OUTPUT_FILE)
        .unwrap();

    let output_file = std::env::args()
        .nth(2)
        .unwrap_or(PARSER_DEFAULT_OUTPUT_FILE.to_string());

    std::fs::write(output_file, result.join("\n")).unwrap();
}
