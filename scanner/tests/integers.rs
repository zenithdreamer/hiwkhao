use scanner_lib;

#[test]
fn zero() {
    let input = "0";
    let expected_output = vec!["0/INT"];
    let output = scanner_lib::run_scanner(input);
    assert_eq!(output, expected_output);
}

#[test]
fn positive_integers() {
    let input = "1 2 3 4 5 6 7 8 9 10";
    let expected_output = vec!["1/INT 2/INT 3/INT 4/INT 5/INT 6/INT 7/INT 8/INT 9/INT 10/INT"];
    let output = scanner_lib::run_scanner(input);
    assert_eq!(output, expected_output);
}

#[test]
fn negative_integers() {
    let input = "-1 -2 -3 -4 -5 -6 -7 -8 -9 -10";
    let expected_output =
        vec!["-1/INT -2/INT -3/INT -4/INT -5/INT -6/INT -7/INT -8/INT -9/INT -10/INT"];
    let output = scanner_lib::run_scanner(input);
    assert_eq!(output, expected_output);
}

#[test]
fn positive_integers_with_whitespace() {
    let input = "1  2  3  4  5  6  7  8  9  10";
    let expected_output = vec!["1/INT 2/INT 3/INT 4/INT 5/INT 6/INT 7/INT 8/INT 9/INT 10/INT"];
    let output = scanner_lib::run_scanner(input);
    assert_eq!(output, expected_output);
}

#[test]
fn negative_integers_with_whitespace() {
    let input = "-1  -2  -3  -4  -5  -6  -7  -8  -9  -10";
    let expected_output =
        vec!["-1/INT -2/INT -3/INT -4/INT -5/INT -6/INT -7/INT -8/INT -9/INT -10/INT"];
    let output = scanner_lib::run_scanner(input);
    assert_eq!(output, expected_output);
}

#[test]
fn positive_integers_with_newline() {
    let input = "1\n2\n3\n4\n5\n6\n7\n8\n9\n10";
    let expected_output = vec![
        "1/INT", "2/INT", "3/INT", "4/INT", "5/INT", "6/INT", "7/INT", "8/INT", "9/INT", "10/INT",
    ];
    let output = scanner_lib::run_scanner(input);
    assert_eq!(output, expected_output);
}

#[test]
fn negative_integers_with_newline() {
    let input = "-1\n-2\n-3\n-4\n-5\n-6\n-7\n-8\n-9\n-10";
    let expected_output = vec![
        "-1/INT", "-2/INT", "-3/INT", "-4/INT", "-5/INT", "-6/INT", "-7/INT", "-8/INT", "-9/INT",
        "-10/INT",
    ];
    let output = scanner_lib::run_scanner(input);
    assert_eq!(output, expected_output);
}
