use scanner;

#[test]
fn ascii_lowercase() {
    let input = "a b c d e f g h i j k l m n o p q r s t u v w x y z";
    let expected_output = vec![
        "a/VAR b/VAR c/VAR d/VAR e/VAR f/VAR g/VAR h/VAR i/VAR j/VAR k/VAR l/VAR m/VAR n/VAR o/VAR p/VAR q/VAR r/VAR s/VAR t/VAR u/VAR v/VAR w/VAR x/VAR y/VAR z/VAR",
    ];
    let output = scanner::run_scanner(input);
    assert_eq!(output, expected_output);
}

#[test]
fn ascii_uppercase() {
    let input = "A B C D E F G H I J K L M N O P Q R S T U V W X Y Z";
    let expected_output = vec![
        "A/VAR B/VAR C/VAR D/VAR E/VAR F/VAR G/VAR H/VAR I/VAR J/VAR K/VAR L/VAR M/VAR N/VAR O/VAR P/VAR Q/VAR R/VAR S/VAR T/VAR U/VAR V/VAR W/VAR X/VAR Y/VAR Z/VAR",
    ];
    let output = scanner::run_scanner(input);
    assert_eq!(output, expected_output);
}

#[test]
fn word_characters() {
    let input = "aA bB cC dD eE fF gG hH iI jJ kK lL mM nN oO pP qQ rR sS tT uU vV wW xX yY zZ";
    let expected_output = vec![
        "aA/VAR bB/VAR cC/VAR dD/VAR eE/VAR fF/VAR gG/VAR hH/VAR iI/VAR jJ/VAR kK/VAR lL/VAR mM/VAR nN/VAR oO/VAR pP/VAR qQ/VAR rR/VAR sS/VAR tT/VAR uU/VAR vV/VAR wW/VAR xX/VAR yY/VAR zZ/VAR",
    ];
    let output = scanner::run_scanner(input);
    assert_eq!(output, expected_output);
}

#[test]
fn invalid_characters() {
    let input = "1a 2b 3c 4d 5e 6f 7g 8h 9i 0j";
    let expected_output = vec![
        "1/INT a/VAR 2/INT b/VAR 3/INT c/VAR 4/INT d/VAR 5/INT e/VAR 6/INT f/VAR 7/INT g/VAR 8/INT h/VAR 9/INT i/VAR 0/INT j/VAR",
    ];
    let output = scanner::run_scanner(input);
    assert_eq!(output, expected_output);
}
