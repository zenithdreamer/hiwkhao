use parser_lib;
use scanner_lib;

#[test]
fn positive_addition() {
    let input = "23 + 8";
    let expected_output = vec![r"(23+8)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser_lib::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn positive_subtraction() {
    let input = "23 - 8";
    let expected_output = vec![r"(23-8)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser_lib::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn positive_multiplication() {
    let input = "23 * 8";
    let expected_output = vec![r"(23*8)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser_lib::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn positive_division() {
    let input = "23.0 / 8.0";
    let expected_output = vec![r"(23.0/8.0)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser_lib::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn positive_division_integers() {
    let input = "23 / 8";
    let expected_output = vec![r"(23/8)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser_lib::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn positive_exponent() {
    let input = "23 ^ 8";
    let expected_output = vec![r"(23^8)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser_lib::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn positive_exponent_float() {
    let input = "23.0 ^ 8.0";
    let expected_output = vec![r"(23.0^8.0)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser_lib::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn negative_addition() {
    let input = "-23 + 8";
    let expected_output = vec![r"((-23)+8)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser_lib::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn negative_addition_no_space() {
    let input = "-23+8";
    let expected_output = vec![r"((-23)+8)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser_lib::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn negative_subtraction() {
    let input = "-23 - 8";
    let expected_output = vec![r"((-23)-8)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser_lib::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn negative_subtraction_no_space_1() {
    let input = "-23-8";
    let expected_output = vec![r"((-23)-8)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser_lib::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn negative_subtraction_no_space_2() {
    let input = "23-8";
    let expected_output = vec![r"(23-8)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser_lib::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn negative_subtraction_no_space_3() {
    let input = "- 23 - 8";
    let expected_output = vec![r"(-(23-8))"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser_lib::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn negative_multiplication() {
    let input = "-23 * 8";
    let expected_output = vec![r"((-23)*8)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser_lib::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn negative_division() {
    let input = "-23.0 / 8.0";
    let expected_output = vec![r"((-23.0)/8.0)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser_lib::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn negative_division_integers() {
    let input = "-23 / 8";
    let expected_output = vec![r"((-23)/8)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser_lib::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn zero_division() {
    let input = "23 / 0";
    let expected_output = vec![r"Division by zero at line 1, pos 4"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser_lib::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn negative_exponent() {
    let input = "-23 ^ 8";
    let expected_output = vec![r"((-23)^8)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser_lib::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn negative_exponent_float() {
    let input = "-23.0 ^ 8.0";
    let expected_output = vec![r"((-23.0)^8.0)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser_lib::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}
