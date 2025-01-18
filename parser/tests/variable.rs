use parser_lib;
use scanner_lib;

#[test]
fn variable_not_declared_1() {
    let input = r"x != y
    ";
    let expected_output = vec![r"Undefined variable x at line 1, pos 1"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser_lib::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn variable_not_declared_2() {
    let input = r"x";
    let expected_output = vec![r"Undefined variable x at line 1, pos 1"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser_lib::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}
