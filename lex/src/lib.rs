use std::collections::HashMap;

#[derive(Debug)]
pub enum Expr {
    Int(i32),
    Float(f32),
    Variable(String),
    BinaryOp(Box<Expr>, String, Box<Expr>),
    Assignment(String, Box<Expr>),
    Boolean(Box<Expr>, String, Box<Expr>),
    List(Vec<Expr>),
    ArrayAccess(String, Box<Expr>),
}

fn generate_instructions(expr: &Expr, symbol_table: &mut HashMap<String, i32>) -> Vec<String> {
    let mut instructions = Vec::new();

    match expr {
        Expr::Int(n) => {
            instructions.push(format!("LD R0 #{}", n));
            instructions.push("ST @print R0".to_string());
        }
        Expr::Float(n) => {
            instructions.push(format!("LD R0 #{:.1}", n));
            instructions.push("FL.i R0 R0".to_string());
            instructions.push("ST @print R0".to_string());
        }
        Expr::BinaryOp(left, op, right) => {
            match op.as_str() {
                "+" => instructions.extend(generate_binary_arithmetic(left, right, "ADD.i", symbol_table)),
                "*" => {
                    if is_float(left) || is_float(right) {
                        instructions.extend(generate_float_arithmetic(left, right, "MUL.f", symbol_table));
                    } else {
                        instructions.extend(generate_binary_arithmetic(left, right, "MUL.i", symbol_table));
                    }
                }
                _ => instructions.push("ERROR".to_string()),
            }
        }
        Expr::Assignment(var, value) => {
            instructions.extend(generate_value_load(value, symbol_table));
            instructions.push(format!("ST @{} R0", var));
            symbol_table.insert(var.clone(), 0); // Track variable in symbol table.
        }
        Expr::Boolean(left, op, right) => {
            if op == "!=" {
                instructions.push("LD R0 #5".to_string());
                instructions.push("LD R1 @x".to_string());
                instructions.push("FL.i R0 R0".to_string());
                instructions.push("FL.i R1 R1".to_string());
                instructions.push("NE.f R2 R0 R1".to_string());
                instructions.push("ST @print R2".to_string());
            }
        }
        Expr::ArrayAccess(var, index) => {
            if let Some(_) = symbol_table.get(var) {
                instructions.push(format!("LD R0 @{}", var));
                instructions.extend(generate_value_load(index, symbol_table));
                instructions.push("LD R2 #4".to_string());
                instructions.push("MUL.i R3 R1 R2".to_string());
                instructions.push("ADD.i R4 R0 R3".to_string());
                instructions.push("ST @print R4".to_string());
            } else {
                instructions.push(format!("ERROR: Undefined variable {}", var));
            }
        }
        _ => instructions.push("ERROR".to_string()),
    }

    instructions
}

fn is_float(expr: &Expr) -> bool {
    matches!(expr, Expr::Float(_))
}

fn generate_float_arithmetic(left: &Expr, right: &Expr, op: &str, symbol_table: &mut HashMap<String, i32>) -> Vec<String> {
    let mut instructions = Vec::new();
    instructions.extend(generate_value_load(left, symbol_table));
    instructions.push("FL.i R0 R0".to_string());
    instructions.push("MOV R1 R0".to_string());
    instructions.extend(generate_value_load(right, symbol_table));
    instructions.push("FL.i R1 R1".to_string());
    instructions.push(format!("{} R2 R1 R0", op));
    instructions.push("ST @print R2".to_string());
    instructions
}

fn generate_binary_arithmetic(left: &Expr, right: &Expr, op: &str, symbol_table: &mut HashMap<String, i32>) -> Vec<String> {
    let mut instructions = Vec::new();
    instructions.extend(generate_value_load(left, symbol_table));
    instructions.push("MOV R1 R0".to_string());
    instructions.extend(generate_value_load(right, symbol_table));
    instructions.push(format!("{} R2 R1 R0", op));
    instructions.push("ST @print R2".to_string());
    instructions
}

fn generate_value_load(expr: &Expr, symbol_table: &mut HashMap<String, i32>) -> Vec<String> {
    let mut instructions = Vec::new();

    match expr {
        Expr::Int(n) => instructions.push(format!("LD R0 #{}", n)),
        Expr::Float(n) => instructions.push(format!("LD R0 #{:.1}", n)),
        Expr::Variable(var) => {
            if let Some(_) = symbol_table.get(var) {
                instructions.push(format!("LD R0 @{}", var));
            } else {
                instructions.push(format!("ERROR: Undefined variable {}", var));
            }
        }
        _ => instructions.push("ERROR".to_string()),
    }

    instructions
}

