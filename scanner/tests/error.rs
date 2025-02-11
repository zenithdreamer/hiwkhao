use scanner;

#[test]
fn special_symbols() {
    let input = "! @ # $ % & |";
    let expected_output = vec!["!/ERR @/ERR #/ERR $/ERR %/ERR &/ERR |/ERR"];
    let output = scanner::run_scanner(input);
    assert_eq!(output, expected_output);
}
