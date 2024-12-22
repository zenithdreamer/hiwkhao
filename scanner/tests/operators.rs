use scanner_lib;

// Positive integers

#[test]
fn positive_addition() {
    let input = "1 + 2";
    let expected_output = vec!["1/INT +/+ 2/INT"];
    let output = scanner_lib::run_scanner(input);
    assert_eq!(output, expected_output);
}

#[test]
fn positive_subtraction() {
    let input = "1 - 2";
    let expected_output = vec!["1/INT -/- 2/INT"];
    let output = scanner_lib::run_scanner(input);
    assert_eq!(output, expected_output);
}

#[test]
fn positive_multiplication() {
    let input = "1 * 2";
    let expected_output = vec!["1/INT */* 2/INT"];
    let output = scanner_lib::run_scanner(input);
    assert_eq!(output, expected_output);
}

#[test]
fn positive_division() {
    let input = "1 / 2";
    let expected_output = vec!["1/INT /// 2/INT"];
    let output = scanner_lib::run_scanner(input);
    assert_eq!(output, expected_output);
}

#[test]
fn positive_division_float() {
    let input = "1.0 / 2.0";
    let expected_output = vec!["1.0/REAL /// 2.0/REAL"];
    let output = scanner_lib::run_scanner(input);
    assert_eq!(output, expected_output);
}

#[test]
fn positive_division_integers() {
    let input = "1 // 2";
    let expected_output = vec!["1/INT ///// 2/INT"];
    let output = scanner_lib::run_scanner(input);
    assert_eq!(output, expected_output);
}

#[test]
fn positive_exponent() {
    let input = "1 ^ 2";
    let expected_output = vec!["1/INT ^/POW 2/INT"];
    let output = scanner_lib::run_scanner(input);
    assert_eq!(output, expected_output);
}

#[test]
fn positive_exponent_float() {
    let input = "1.0 ^ 2.0";
    let expected_output = vec!["1.0/REAL ^/POW 2.0/REAL"];
    let output = scanner_lib::run_scanner(input);
    assert_eq!(output, expected_output);
}

#[test]
fn positive_greater_than() {
    let input = "1 > 2";
    let expected_output = vec!["1/INT >/> 2/INT"];
    let output = scanner_lib::run_scanner(input);
    assert_eq!(output, expected_output);
}

#[test]
fn positive_greater_than_or_equal() {
    let input = "1 >= 2";
    let expected_output = vec!["1/INT >=/>= 2/INT"];
    let output = scanner_lib::run_scanner(input);
    assert_eq!(output, expected_output);
}

#[test]
fn positive_less_than() {
    let input = "1 < 2";
    let expected_output = vec!["1/INT </< 2/INT"];
    let output = scanner_lib::run_scanner(input);
    assert_eq!(output, expected_output);
}

#[test]
fn positive_less_than_or_equal() {
    let input = "1 <= 2";
    let expected_output = vec!["1/INT <=/<= 2/INT"];
    let output = scanner_lib::run_scanner(input);
    assert_eq!(output, expected_output);
}

#[test]
fn positive_equal() {
    let input = "1 == 2";
    let expected_output = vec!["1/INT ==/== 2/INT"];
    let output = scanner_lib::run_scanner(input);
    assert_eq!(output, expected_output);
}

#[test]
fn positive_not_equal() {
    let input = "1 != 2";
    let expected_output = vec!["1/INT !=/!= 2/INT"];
    let output = scanner_lib::run_scanner(input);
    assert_eq!(output, expected_output);
}

#[test]
fn positive_parentheses() {
    let input = "(1 + 2)";
    let expected_output = vec!["(/LPAREN 1/INT +/+ 2/INT )/RPAREN"];
    let output = scanner_lib::run_scanner(input);
    assert_eq!(output, expected_output);
}

#[test]
fn positive_assignment() {
    let input = "a = 1";
    let expected_output = vec!["a/VAR =/= 1/INT"];
    let output = scanner_lib::run_scanner(input);
    assert_eq!(output, expected_output);
}

// Negative integers

#[test]
fn negative_addition() {
    let input = "-1 + -2";
    let expected_output = vec!["-/- 1/INT +/+ -/- 2/INT"];
    let output = scanner_lib::run_scanner(input);
    assert_eq!(output, expected_output);
}

#[test]
fn negative_subtraction() {
    let input = "-1 - -2";
    let expected_output = vec!["-/- 1/INT -/- -/- 2/INT"];
    let output = scanner_lib::run_scanner(input);
    assert_eq!(output, expected_output);
}

#[test]
fn negative_multiplication() {
    let input = "-1 * -2";
    let expected_output = vec!["-/- 1/INT */* -/- 2/INT"];
    let output = scanner_lib::run_scanner(input);
    assert_eq!(output, expected_output);
}

#[test]
fn negative_division() {
    let input = "-1 / -2";
    let expected_output = vec!["-/- 1/INT /// -/- 2/INT"];
    let output = scanner_lib::run_scanner(input);
    assert_eq!(output, expected_output);
}

#[test]
fn negative_division_float() {
    let input = "-1.0 / -2.0";
    let expected_output = vec!["-/- 1.0/REAL /// -/- 2.0/REAL"];
    let output = scanner_lib::run_scanner(input);
    assert_eq!(output, expected_output);
}

#[test]
fn negative_division_integers() {
    let input = "-1 // -2";
    let expected_output = vec!["-/- 1/INT ///// -/- 2/INT"];
    let output = scanner_lib::run_scanner(input);
    assert_eq!(output, expected_output);
}

#[test]
fn negative_exponent() {
    let input = "-1 ^ -2";
    let expected_output = vec!["-/- 1/INT ^/POW -/- 2/INT"];
    let output = scanner_lib::run_scanner(input);
    assert_eq!(output, expected_output);
}

#[test]
fn negative_exponent_float() {
    let input = "-1.0 ^ -2.0";
    let expected_output = vec!["-/- 1.0/REAL ^/POW -/- 2.0/REAL"];
    let output = scanner_lib::run_scanner(input);
    assert_eq!(output, expected_output);
}

#[test]
fn negative_greater_than() {
    let input = "-1 > -2";
    let expected_output = vec!["-/- 1/INT >/> -/- 2/INT"];
    let output = scanner_lib::run_scanner(input);
    assert_eq!(output, expected_output);
}

#[test]
fn negative_greater_than_or_equal() {
    let input = "-1 >= -2";
    let expected_output = vec!["-/- 1/INT >=/>= -/- 2/INT"];
    let output = scanner_lib::run_scanner(input);
    assert_eq!(output, expected_output);
}

#[test]
fn negative_less_than() {
    let input = "-1 < -2";
    let expected_output = vec!["-/- 1/INT </< -/- 2/INT"];
    let output = scanner_lib::run_scanner(input);
    assert_eq!(output, expected_output);
}

#[test]
fn negative_less_than_or_equal() {
    let input = "-1 <= -2";
    let expected_output = vec!["-/- 1/INT <=/<= -/- 2/INT"];
    let output = scanner_lib::run_scanner(input);
    assert_eq!(output, expected_output);
}

#[test]
fn negative_equal() {
    let input = "-1 == -2";
    let expected_output = vec!["-/- 1/INT ==/== -/- 2/INT"];
    let output = scanner_lib::run_scanner(input);
    assert_eq!(output, expected_output);
}

#[test]
fn negative_not_equal() {
    let input = "-1 != -2";
    let expected_output = vec!["-/- 1/INT !=/!= -/- 2/INT"];
    let output = scanner_lib::run_scanner(input);
    assert_eq!(output, expected_output);
}

#[test]
fn negative_parentheses() {
    let input = "(-1 + -2)";
    let expected_output = vec!["(/LPAREN -/- 1/INT +/+ -/- 2/INT )/RPAREN"];
    let output = scanner_lib::run_scanner(input);
    assert_eq!(output, expected_output);
}

#[test]
fn negative_assignment() {
    let input = "a = -1";
    let expected_output = vec!["a/VAR =/= -/- 1/INT"];
    let output = scanner_lib::run_scanner(input);
    assert_eq!(output, expected_output);
}
