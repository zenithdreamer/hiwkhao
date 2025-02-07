use parser_lib::Expr;
use codegen;

#[test]
fn test_integer_addition() {
    let expr = Expr::BinaryOp(
        Box::new(Expr::Int(23)),
        String::from("+"),
        Box::new(Expr::Int(8))
    );
    let expected = vec![
        "LD R0 #23",
        "LD R1 #8",
        "ADD.i R2 R0 R1",
        "ST @print R2"
    ];
    assert_eq!(codegen::generate_assembly(&expr), expected);
}

#[test]
fn test_float_multiplication() {
    let expr = Expr::BinaryOp(
        Box::new(Expr::Float(2.5)),
        String::from("*"),
        Box::new(Expr::Int(0))
    );
    let expected = vec![
        "LD R0 #2.5",
        "LD R1 #0",
        "FL.i R1 R1",
        "MUL.f R2 R0 R1",
        "ST @print R2"
    ];
    assert_eq!(codegen::generate_assembly(&expr), expected);
}

#[test]
fn test_variable_assignment() {
    let expr = Expr::Assignment(
        String::from("x"),
        Box::new(Expr::Int(5))
    );
    let expected = vec![
        "LD R0 #5",
        "ST @x R0"
    ];
    assert_eq!(codegen::generate_assembly(&expr), expected);
}

#[test]
fn test_variable_multiplication() {
    let expr = Expr::BinaryOp(
        Box::new(Expr::Int(10)),
        String::from("*"),
        Box::new(Expr::Variable(String::from("x")))
    );
    let expected = vec![
        "LD R0 #10",
        "LD R1 @x",
        "MUL.i R2 R0 R1",
        "ST @print R2"
    ];
    assert_eq!(codegen::generate_assembly(&expr), expected);
}

#[test]
fn test_invalid_operation() {
    let expr = Expr::BinaryOp(
        Box::new(Expr::Int(5)),
        String::from("^"),
        Box::new(Expr::Int(2))
    );
    let expected = vec!["ERROR"];
    assert_eq!(codegen::generate_assembly(&expr), expected);
}

#[test]
fn test_undefined_variable() {
    let expr = Expr::Assignment(
        String::from("x"),
        Box::new(Expr::Variable(String::from("y")))
    );
    let expected = vec!["ERROR"];
    assert_eq!(codegen::generate_assembly(&expr), expected);
}

#[test]
fn test_list_assignment() {
    let expr = Expr::Assignment(
        String::from("x"),
        Box::new(Expr::List(vec![]))
    );
    let expected = vec![
        "LD R0 #0",
        "LD R1 @x",
        "LD R2 #0",
        "LD R3 #4",
        "MUL.i R4 R2 R3",
        "ADD.i R5 R1 R4",
        "ST R5 R0",
        "LD R2 #1",
        "LD R3 #4",
        "MUL.i R4 R2 R3",
        "ADD.i R5 R1 R4",
        "ST R5 R0"
    ];
    assert_eq!(codegen::generate_assembly(&expr), expected);
}

#[test]
fn test_list_element_access() {
    let expr = Expr::BinaryOp(
        Box::new(Expr::Variable(String::from("x"))),
        String::from("[]"),
        Box::new(Expr::Int(1))
    );
    let expected = vec!["ERROR"];
    assert_eq!(codegen::generate_assembly(&expr), expected);
}

#[test]
fn test_float_int_multiplication() {
    let expr = Expr::BinaryOp(
        Box::new(Expr::Float(2.5)),
        String::from("*"),
        Box::new(Expr::Int(3))
    );
    let expected = vec![
        "LD R0 #2.5",
        "LD R1 #3",
        "FL.i R1 R1",
        "MUL.f R2 R0 R1",
        "ST @print R2"
    ];
    assert_eq!(codegen::generate_assembly(&expr), expected);
}

#[test]
fn test_invalid_float_operation() {
    let expr = Expr::BinaryOp(
        Box::new(Expr::Float(2.5)),
        String::from("-"),
        Box::new(Expr::Int(3))
    );
    let expected = vec!["ERROR"];
    assert_eq!(codegen::generate_assembly(&expr), expected);
}

#[test]
fn test_invalid_list_operation() {
    let expr = Expr::BinaryOp(
        Box::new(Expr::List(vec![])),
        String::from("+"),
        Box::new(Expr::Int(1))
    );
    let expected = vec!["ERROR"];
    assert_eq!(codegen::generate_assembly(&expr), expected);
}

#[test]
fn test_unsupported_operation() {
    let expr = Expr::BinaryOp(
        Box::new(Expr::Int(5)),
        String::from("&"),
        Box::new(Expr::Int(2))
    );
    let expected = vec!["ERROR"];
    assert_eq!(codegen::generate_assembly(&expr), expected);
} 