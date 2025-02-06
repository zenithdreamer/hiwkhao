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
            match value.as_ref() {
                Expr::List(elements) => {
                    // Handle list assignment
                    instructions.push("LD R0 #0".to_string()); // Base address for the list
                    for (i, elem) in elements.iter().enumerate() {
                        instructions.extend(generate_value_load(elem, symbol_table));
                        instructions.push(format!("ST @{}_{} R0", var, i));
                    }
                    symbol_table.insert(var.clone(), elements.len() as i32);
                }
                _ => {
                    instructions.extend(generate_value_load(value, symbol_table));
                    instructions.push(format!("ST @{} R0", var));
                    symbol_table.insert(var.clone(), 0);
                }
            }
        }
        Expr::Boolean(left, op, right) => {
            if op == "!=" {
                instructions.extend(generate_value_load(left, symbol_table));
                instructions.push("MOV R1 R0".to_string());
                instructions.extend(generate_value_load(right, symbol_table));
                instructions.push("NE.i R2 R1 R0".to_string());
                instructions.push("ST @print R2".to_string());
            }
        }
        Expr::ArrayAccess(var, index) => {
            if let Some(&size) = symbol_table.get(var) {
                instructions.extend(generate_value_load(index, symbol_table));
                instructions.push(format!("LD R1 @{}_{}", var, 0)); // Access the indexed element
                instructions.push("ST @print R1".to_string());
            } else {
                instructions.push(format!("ERROR: Undefined variable {}", var));
            }
        }
        _ => instructions.push("ERROR".to_string()),
    }

    instructions
}
