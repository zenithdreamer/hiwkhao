use scanner_lib;

const DEFAULT_OUTPUT_FILE: &str = "hiwkhao.tok";

// TODO: Since we only have scanner right now, we run the scanner for now.
fn main() {
    let input = if let Some(file_path) = std::env::args().nth(1) {
        std::fs::read_to_string(file_path).unwrap()
    } else {
        eprintln!("No input file provided.");
        std::process::exit(1);
    };

    let result = scanner_lib::run_scanner(&input);
    println!("{}", result.join("\n"));

    let output_file = std::env::args()
        .nth(2)
        .unwrap_or(DEFAULT_OUTPUT_FILE.to_string());

    std::fs::write(output_file, result.join("\n")).unwrap();
}
