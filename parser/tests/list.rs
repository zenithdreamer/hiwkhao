#[test]
fn list_with_index() {
    let input = "list[5]";
    let expected_output = vec![r"(list[5])"];  // Adjust expected output based on your parser's format
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn list_with_expression_index() {
    let input = "list[1+2]";
    let expected_output = vec![r"SyntaxError at line 1, pos 6"];  // Adjust expected output based on your parser's format
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn invalid_list_index() {
    let input = "list[]";
    let expected_output = vec![r"Missing index expression at line 1, pos 5"];  // Adjust error message as needed
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn list_with_arithmetic() {
    let input = "x = list[2] + 5";
    let expected_output = vec![r"SyntaxError at line 1, pos 11"];  // Error when trying to perform arithmetic with list
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn list_with_arithmetic2() {
    let input = "x = 5 + list[2]";
    let expected_output = vec![r"SyntaxError at line 1, pos 11"];  // Error when trying to perform arithmetic with list
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn list_with_negative_index() {
    let input = "list[-1]";
    let expected_output = vec![r"SyntaxError at line 1, pos 6"];  // Error for negative index
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}
