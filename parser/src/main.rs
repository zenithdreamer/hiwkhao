use parser_lib;

const DEFAULT_OUTPUT_FILE: &str = "hiwkhao.tok";

fn main() {
    let input = if let Some(file_path) = std::env::args().nth(1) {
        std::fs::read_to_string(file_path).unwrap()
    } else {
        eprintln!("No input file provided.");
        std::process::exit(1);
    };

    let scanner_result = parser_lib::run_scanner(&input);
    println!("{}", scanner_result.join("\n"));

    println!();

    let parser_result = parser_lib::run_parser(&scanner_result.join("\n"));
    println!("{}", parser_result.join("\n"));

    let output_file = std::env::args()
        .nth(2)
        .unwrap_or(DEFAULT_OUTPUT_FILE.to_string());

    std::fs::write(output_file, scanner_result.join("\n")).unwrap();
}
