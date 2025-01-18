use parser::symbol_table::SymbolTable;
use parser::Parser;

const SCANNER_DEFAULT_OUTPUT_FILE: &str = "hiwkhao.tok";
const SYMBOL_TABLE_DEFAULT_OUTPUT_FILE: &str = "hiwkhao.csv";

fn scanner(input: &String) {
    let result = scanner_lib::run_scanner(&input);
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

    let tokens = scanner_lib::tokenize(&input);
    let mut parser = Parser::new(vec![]);

    parser.parse_tokens(tokens.clone());

    let mut table = SymbolTable::new(vec![]);
    table.get_symbol_table(tokens);
    table
        .write_to_csv(SYMBOL_TABLE_DEFAULT_OUTPUT_FILE)
        .unwrap();

    //println!("{:?}", table);

    //parser = Parser::new(vec![]);
    //result = parser.parse_file(input);

    //println!("{:?}", result);
}
