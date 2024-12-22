use scanner_lib;

#[test]
fn special_symbols() {
    let input = "! @ # $ % & |";
    let expected_output = vec!["!/ERR @/ERR #/ERR $/ERR %/ERR &/ERR |/ERR"];
    let output = scanner_lib::run_scanner(input);
    assert_eq!(output, expected_output);
}
