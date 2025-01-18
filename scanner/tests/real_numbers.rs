use scanner_lib;

#[test]
fn zero_real_number() {
    let input = "0.0";
    let expected_output = vec!["0.0/REAL"];
    let output = scanner_lib::run_scanner(input);
    assert_eq!(output, expected_output);
}

#[test]
fn positive_real_numbers() {
    let input = "1.0 2.0 3.0 4.0 5.0 6.0 7.0 8.0 9.0 10.0";
    let expected_output = vec!["1.0/REAL 2.0/REAL 3.0/REAL 4.0/REAL 5.0/REAL 6.0/REAL 7.0/REAL 8.0/REAL 9.0/REAL 10.0/REAL"];
    let output = scanner_lib::run_scanner(input);
    assert_eq!(output, expected_output);
}

#[test]
fn positive_real_numbers_with_whitespace() {
    let input = "1.0  2.0  3.0  4.0  5.0  6.0  7.0  8.0  9.0  10.0";
    let expected_output = vec!["1.0/REAL 2.0/REAL 3.0/REAL 4.0/REAL 5.0/REAL 6.0/REAL 7.0/REAL 8.0/REAL 9.0/REAL 10.0/REAL"];
    let output = scanner_lib::run_scanner(input);
    assert_eq!(output, expected_output);
}

#[test]
fn positive_real_numbers_with_newline() {
    let input = "1.0\n2.0\n3.0\n4.0\n5.0\n6.0\n7.0\n8.0\n9.0\n10.0";
    let expected_output = vec![
        "1.0/REAL",
        "2.0/REAL",
        "3.0/REAL",
        "4.0/REAL",
        "5.0/REAL",
        "6.0/REAL",
        "7.0/REAL",
        "8.0/REAL",
        "9.0/REAL",
        "10.0/REAL",
    ];
    let output = scanner_lib::run_scanner(input);
    assert_eq!(output, expected_output);
}

#[test]
fn negative_real_numbers() {
    let input = "-1.0 -2.0 -3.0 -4.0 -5.0 -6.0 -7.0 -8.0 -9.0 -10.0";
    let expected_output = vec!["-1.0/REAL -2.0/REAL -3.0/REAL -4.0/REAL -5.0/REAL -6.0/REAL -7.0/REAL -8.0/REAL -9.0/REAL -10.0/REAL"];
    let output = scanner_lib::run_scanner(input);
    assert_eq!(output, expected_output);
}

#[test]
fn negative_real_numbers_with_whitespace() {
    let input = "-1.0  -2.0  -3.0  -4.0  -5.0  -6.0  -7.0  -8.0  -9.0  -10.0";
    let expected_output = vec!["-1.0/REAL -2.0/REAL -3.0/REAL -4.0/REAL -5.0/REAL -6.0/REAL -7.0/REAL -8.0/REAL -9.0/REAL -10.0/REAL"];
    let output = scanner_lib::run_scanner(input);
    assert_eq!(output, expected_output);
}

#[test]
fn negative_real_numbers_with_newline() {
    let input = "-1.0\n-2.0\n-3.0\n-4.0\n-5.0\n-6.0\n-7.0\n-8.0\n-9.0\n-10.0";
    let expected_output = vec![
        "-1.0/REAL",
        "-2.0/REAL",
        "-3.0/REAL",
        "-4.0/REAL",
        "-5.0/REAL",
        "-6.0/REAL",
        "-7.0/REAL",
        "-8.0/REAL",
        "-9.0/REAL",
        "-10.0/REAL",
    ];
    let output = scanner_lib::run_scanner(input);
    assert_eq!(output, expected_output);
}
