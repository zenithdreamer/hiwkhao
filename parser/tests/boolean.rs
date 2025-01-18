use scanner_lib;

#[test]
fn positive_greater_than_or_equal_integers() {
    let input = "23 >= 8";
    let expected_output = vec![r"(23>=8)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn positive_greater_than_or_equal_floats() {
    let input = "23.0 >= 8.0";
    let expected_output = vec![r"(23.0>=8.0)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn positive_greater_than_or_equal_variables() {
    let input = r"x = 2
    y = 3
    x >= y
    ";
    let expected_output = vec![r"(x>=y)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn positive_less_than_or_equal_integers() {
    let input = "23 <= 8";
    let expected_output = vec![r"(23<=8)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn positive_less_than_or_equal_floats() {
    let input = "23.0 <= 8.0";
    let expected_output = vec![r"(23.0<=8.0)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn positive_less_than_or_equal_variables() {
    let input = r"x = 2
    y = 3
    x <= y
    ";
    let expected_output = vec![r"(x<=y)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn positive_less_than() {
    let input = "23 < 8";
    let expected_output = vec![r"(23<8)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn positive_less_than_variables() {
    let input = r"x = 2
    y = 3
    x < y
    ";
    let expected_output = vec![r"(x<y)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn positive_greater_than() {
    let input = "23 > 8";
    let expected_output = vec![r"(23>8)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn positive_greater_than_variables() {
    let input = r"x = 2
    y = 3
    x > y
    ";
    let expected_output = vec![r"(x>y)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn positive_equal_integers() {
    let input = "23 == 8";
    let expected_output = vec![r"(23==8)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn positive_equal_floats() {
    let input = "23.0 == 8.0";
    let expected_output = vec![r"(23.0==8.0)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn positive_equal_variables() {
    let input = r"x = 2
    y = 3
    x == y
    ";
    let expected_output = vec![r"(x==y)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn positive_not_equal_integers() {
    let input = "23 != 8";
    let expected_output = vec![r"(23!=8)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn positive_not_equal_floats() {
    let input = "23.0 != 8.0";
    let expected_output = vec![r"(23.0!=8.0)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn positive_not_equal_variables() {
    let input = r"x = 2
    y = 3
    x != y
    ";
    let expected_output = vec![r"(x!=y)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn negative_greater_than_or_equal_integers() {
    let input = "-23 >= 8";
    let expected_output = vec![r"((-23)>=8)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn negative_greater_than_or_equal_floats() {
    let input = "-23.0 >= 8.0";
    let expected_output = vec![r"((-23.0)>=8.0)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn negative_greater_than_or_equal_variables() {
    let input = r"x = 2
    y = 3
    -x >= y
    ";
    let expected_output = vec![r"((-x)>=y)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn negative_less_than_or_equal_integers() {
    let input = "-23 <= 8";
    let expected_output = vec![r"((-23)<=8)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn negative_less_than_or_equal_floats() {
    let input = "-23.0 <= 8.0";
    let expected_output = vec![r"((-23.0)<=8.0)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn negative_less_than_or_equal_variables() {
    let input = r"x = 2
    y = 3
    -x <= y
    ";
    let expected_output = vec![r"((-x)<=y)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn negative_less_than() {
    let input = "-23 < 8";
    let expected_output = vec![r"((-23)<8)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn negative_less_than_variables() {
    let input = r"x = 2
    y = 3
    -x < y
    ";
    let expected_output = vec![r"((-x)<y)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn negative_greater_than() {
    let input = "-23 > 8";
    let expected_output = vec![r"((-23)>8)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn negative_greater_than_variables() {
    let input = r"x = 2
    y = 3
    -x > y
    ";
    let expected_output = vec![r"((-x)>y)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn negative_equal_integers() {
    let input = "-23 == 8";
    let expected_output = vec![r"((-23)==8)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn negative_equal_floats() {
    let input = "-23.0 == 8.0";
    let expected_output = vec![r"((-23.0)==8.0)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn negative_equal_variables() {
    let input = r"x = 2
    y = 3
    -x == y
    ";
    let expected_output = vec![r"((-x)==y)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn negative_not_equal_integers() {
    let input = "-23 != 8";
    let expected_output = vec![r"((-23)!=8)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn negative_not_equal_floats() {
    let input = "23.0 != 8.0";
    let expected_output = vec![r"(23.0!=8.0)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}

#[test]
fn negative_not_equal_variables() {
    let input = r"x = 2
    y = 3
    -x != y
    ";
    let expected_output = vec![r"((-x)!=y)"];
    let tokens = scanner_lib::tokenize(input);
    let mut parser = parser::Parser::new(vec![]);
    let output = parser.parse_tokens_fancy(tokens);
    assert_eq!(output, expected_output);
}
