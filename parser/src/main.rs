use scanner_lib::grammar::Token;

mod symbol_table;

fn main() {
    let input = if let Some(file_path) = std::env::args().nth(1) {
        std::fs::read_to_string(file_path).unwrap()
    } else {
        eprintln!("No input file provided.");
        std::process::exit(1);
    };

    // let result = parser::parse_input(&input);

    println!("lexeme, line number, startpos, length, value type, value");

    let results = symbol_table::get_symbol_table(&input);
    
    for entry in results.output() {
        println!("{}", entry);
    }
}
