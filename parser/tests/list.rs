use parser_lib;
use scanner_lib;

#[test]
fn list_with_index() {
    let input = "list[5]";
    let expected_output = vec![r"(list[(5)])"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser_lib::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn list_with_expression_index() {
    let input = "list[1+2]";
    let expected_output = vec![r"SyntaxError at line 1, pos 6"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser_lib::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn invalid_list_index() {
    let input = "list[]";
    let expected_output = vec![r"Missing index expression at line 1, pos 5"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser_lib::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn list_with_arithmetic() {
    let input = "x = list[2] + 5";
    let expected_output = vec![r"SyntaxError at line 1, pos 11"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser_lib::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn list_with_arithmetic2() {
    let input = "x = 5 + list[2]";
    let expected_output = vec![r"SyntaxError at line 1, pos 11"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser_lib::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn list_with_negative_index() {
    let input = "list[-1]";
    let expected_output = vec![r"SyntaxError at line 1, pos 6"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser_lib::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn list_index_assignment_integer() {
    let input = r"x = list[2]
x[0] = 5";
    let expected_output = vec!["(x=(list[(2)]))", "(x[(0)]=5)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser_lib::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn list_index_assignment_real() {
    let input = r"x = list[2]
x[0] = 5.0";
    let expected_output = vec!["(x=(list[(2)]))", "(x[(0)]=5.0)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser_lib::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn list_index_assignment_variable() {
    let input = "x = list[2]";
    let expected_output = vec![r"(x=(list[(2)]))"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser_lib::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn list_index_assignment_list_indices() {
    let input = r"x = list[2]
y = list[3]
x[0] = y[0]
";
    let expected_output = vec!["(x=(list[(2)]))", "(y=(list[(3)]))", "(x[(0)]=(y[(0)]))"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser_lib::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}
