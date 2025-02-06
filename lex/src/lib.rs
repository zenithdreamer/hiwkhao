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

fn generate_instructions(expr: &Expr) -> Vec<String> {
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
                "+" => {
                    instructions.extend(generate_binary_arithmetic(left, right, "ADD.i"));
                }
                "*" => {
                    if is_float(left) || is_float(right) {
                        instructions.extend(generate_float_arithmetic(left, right, "MUL.f"));
                    } else {
                        instructions.extend(generate_binary_arithmetic(left, right, "MUL.i"));
                    }
                }
                _ => instructions.push("ERROR".to_string()),
            }
        }
        Expr::Assignment(var, value) => {
            match &**value {
                Expr::List(elements) => {
                    instructions.push(format!("LD R0 #0 // Initialize array {}", var));
                    instructions.push(format!("LD R1 @{}", var));
                    for i in 0..elements.len() {
                        instructions.push(format!("LD R2 #{}", i));
                        instructions.push("LD R3 #4".to_string());
                        instructions.push("MUL.i R4 R2 R3".to_string());
                        instructions.push("ADD.i R5 R1 R4".to_string());
                        instructions.push("ST R5 R0".to_string());
                    }
                }
                _ => {
                    instructions.extend(generate_value_load(value));
                    instructions.push(format!("ST @{} R0", var));
                }
            }
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
            instructions.push(format!("LD R0 @{}", var));
            instructions.extend(generate_value_load(index));
            instructions.push("LD R2 #4".to_string());
            instructions.push("MUL.i R3 R1 R2".to_string());
            instructions.push("ADD.i R4 R0 R3".to_string());
            instructions.push("ST @print R4".to_string());
        }
        _ => {
            instructions.push("ERROR".to_string());
        }
    }
    
    instructions
}

fn is_float(expr: &Expr) -> bool {
    matches!(expr, Expr::Float(_))
}

fn generate_float_arithmetic(left: &Expr, right: &Expr, op: &str) -> Vec<String> {
    let mut instructions = Vec::new();
    instructions.extend(generate_value_load(left));
    instructions.push("FL.i R0 R0".to_string());
    instructions.push("MOV R1 R0".to_string());
    instructions.extend(generate_value_load(right));
    instructions.push("FL.i R1 R1".to_string());
    instructions.push(format!("{} R2 R1 R0", op));
    instructions.push("ST @print R2".to_string());
    instructions
}