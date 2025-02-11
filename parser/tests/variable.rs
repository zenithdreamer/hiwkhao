use parser;
use scanner;

#[test]
fn variable_not_declared_1() {
    let input = r"x != y
    ";
    let expected_output = vec![r"Undefined variable x at line 1, pos 1"];
    let tokens = scanner::tokenize(input);
    let mut parser = parser::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn variable_not_declared_2() {
    let input = r"x";
    let expected_output = vec![r"Undefined variable x at line 1, pos 1"];
    let tokens = scanner::tokenize(input);
    let mut parser = parser::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}
