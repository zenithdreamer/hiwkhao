use scanner_lib;


#[test]
fn invalid_assignment() {
    let input = r"2 = 3";
    let expected_output = vec![r"SyntaxError at line 1, pos 1"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}
