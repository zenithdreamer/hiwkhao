use std::collections::HashMap;
use parser_lib::Expr;

struct RegisterAllocator {
    next_reg: i64,
}

impl RegisterAllocator {
    fn new() -> Self {
        RegisterAllocator { next_reg: 0 }
    }

    fn get_next_reg(&mut self) -> i64 {
        let reg = self.next_reg;
        self.next_reg += 1;
        reg
    }
}

fn generate_binary_arithmetic(left: &Expr, right: &Expr, op: &str, _symbol_table: &mut HashMap<String, i64>, reg_alloc: &mut RegisterAllocator) -> Vec<String> {
    let mut instructions = Vec::new();
    
    match (left, right) {
        (Expr::Int(n1), Expr::Int(n2)) => {
            // Check for division by zero
            if op == "/" && *n2 == 0 {
                instructions.push("ERROR".to_string());
                return instructions;
            }
            
            let r0 = reg_alloc.get_next_reg();
            let r1 = reg_alloc.get_next_reg();
            let r2 = reg_alloc.get_next_reg();
            
            instructions.push(format!("LD R{} #{}", r0, n1));
            instructions.push(format!("LD R{} #{}", r1, n2));
            
            let op_code = match op {
                "+" => "ADD.i",
                "-" => "SUB.i",
                "*" => "MUL.i",
                "/" => "DIV.i",
                "==" => "EQ.i",
                "!=" => "NE.i",
                "<" => "LT.i",
                ">" => "GT.i",
                "<=" => "LE.i",
                ">=" => "GE.i",
                _ => {
                    instructions.push("ERROR".to_string());
                    return instructions;
                }
            };
            
            instructions.push(format!("{} R{} R{} R{}", op_code, r2, r0, r1));
            instructions.push(format!("ST @print R{}", r2));
        }
        (Expr::Int(n1), Expr::Float(n2)) => {
            let r0 = reg_alloc.get_next_reg();
            let r1 = reg_alloc.get_next_reg();
            let r2 = reg_alloc.get_next_reg();
            
            instructions.push(format!("LD R{} #{}", r0, n1));
            instructions.push(format!("FL.i R{} R{}", r0, r0));
            instructions.push(format!("LD R{} #{}", r1, n2));
            
            let op_code = match op {
                "+" => "ADD.f",
                "-" => "SUB.f",
                "*" => "MUL.f",
                "/" => "DIV.f",
                "==" => "EQ.f",
                "!=" => "NE.f",
                "<" => "LT.f",
                ">" => "GT.f",
                "<=" => "LE.f",
                ">=" => "GE.f",
                _ => {
                    instructions.push("ERROR".to_string());
                    return instructions;
                }
            };
            
            instructions.push(format!("{} R{} R{} R{}", op_code, r2, r0, r1));
            instructions.push(format!("ST @print R{}", r2));
        }
        (Expr::Float(n1), Expr::Int(n2)) => {
            let r0 = reg_alloc.get_next_reg();
            let r1 = reg_alloc.get_next_reg();
            let r2 = reg_alloc.get_next_reg();
            
            instructions.push(format!("LD R{} #{}", r0, n1));
            instructions.push(format!("LD R{} #{}", r1, n2));
            instructions.push(format!("FL.i R{} R{}", r1, r1));
            
            let op_code = match op {
                "+" => "ADD.f",
                "-" => "SUB.f",
                "*" => "MUL.f",
                "/" => "DIV.f",
                "==" => "EQ.f",
                "!=" => "NE.f",
                "<" => "LT.f",
                ">" => "GT.f",
                "<=" => "LE.f",
                ">=" => "GE.f",
                _ => {
                    instructions.push("ERROR".to_string());
                    return instructions;
                }
            };
            
            instructions.push(format!("{} R{} R{} R{}", op_code, r2, r0, r1));
            instructions.push(format!("ST @print R{}", r2));
        }
        (Expr::Int(n), Expr::Variable(var)) | (Expr::Variable(var), Expr::Int(n)) => {
            let r0 = reg_alloc.get_next_reg();
            let r1 = reg_alloc.get_next_reg();
            let r2 = reg_alloc.get_next_reg();
            
            // Load the integer and variable values
            instructions.push(format!("LD R{} #{}", r0, n));
            instructions.push(format!("LD R{} @{}", r1, var));
            
            let op_code = match op {
                "+" => "ADD.i",
                "-" => "SUB.i",
                "*" => "MUL.i",
                "/" => "DIV.i",
                "==" => "EQ.i",
                "!=" => "NE.i",
                "<" => "LT.i",
                ">" => "GT.i",
                "<=" => "LE.i",
                ">=" => "GE.i",
                _ => {
                    instructions.push("ERROR".to_string());
                    return instructions;
                }
            };
            
            instructions.push(format!("{} R{} R{} R{}", op_code, r2, r0, r1));
            instructions.push(format!("ST @print R{}", r2));
        }
        _ => instructions.push("ERROR".to_string()),
    }
    
    instructions
}

fn generate_instructions(expr: &Expr, symbol_table: &mut HashMap<String, i64>, reg_alloc: &mut RegisterAllocator) -> Vec<String> {
    println!("DEBUG [Codegen]: Starting instruction generation for expr: {:?}", expr);
    let mut instructions = Vec::new();

    match expr {
        Expr::Int(n) => {
            println!("DEBUG [Codegen]: Generating instructions for integer: {}", n);
            let r0 = reg_alloc.get_next_reg();
            instructions.push(format!("LD R{} #{}", r0, n));
            instructions.push(format!("ST @print R{}", r0));
        }
        Expr::Float(n) => {
            println!("DEBUG [Codegen]: Generating instructions for float: {}", n);
            let r0 = reg_alloc.get_next_reg();
            instructions.push(format!("LD R{} #{}", r0, n));
            instructions.push(format!("ST @print R{}", r0));
        }
        Expr::BinaryOp(left, op, right) => {
            println!("DEBUG [Codegen]: Generating instructions for binary op: {} {:?} {:?}", op, left, right);
            match op.as_str() {
                "+" | "-" | "*" | "/" => {
                    println!("DEBUG [Codegen]: {} operation", match op.as_str() {
                        "+" => "Addition",
                        "-" => "Subtraction",
                        "*" => "Multiplication",
                        "/" => "Division",
                        _ => unreachable!()
                    });
                    instructions.extend(generate_binary_arithmetic(left, right, op, symbol_table, reg_alloc))
                },
                "^" | "POW" => {
                    println!("DEBUG [Codegen]: Power operation detected");
                    instructions.push("ERROR".to_string())
                }
                _ => {
                    println!("DEBUG [Codegen]: Unknown binary operator: {}", op);
                    instructions.push("ERROR".to_string())
                },
            }
        }
        Expr::Assignment(var, value) => {
            match value.as_ref() {
                Expr::Int(n) => {
                    let r0 = reg_alloc.get_next_reg();
                    instructions.push(format!("LD R{} #{}", r0, n));
                    instructions.push(format!("ST @{} R{}", var, r0));
                    symbol_table.insert(var.clone(), *n);
                }
                Expr::List(_) => {
                    println!("DEBUG [Codegen]: Processing List assignment");
                    let r0 = reg_alloc.get_next_reg();
                    let r1 = reg_alloc.get_next_reg();
                    let r2 = reg_alloc.get_next_reg();
                    let r3 = reg_alloc.get_next_reg();
                    let r4 = reg_alloc.get_next_reg();
                    let r5 = reg_alloc.get_next_reg();
                    
                    println!("DEBUG [Codegen]: List assignment registers: r0={}, r1={}, r2={}, r3={}, r4={}, r5={}", r0, r1, r2, r3, r4, r5);
                    
                    // Initialize list elements to 0
                    instructions.push(format!("LD R{} #0", r0));
                    instructions.push(format!("LD R{} @{}", r1, var));
                    
                    // Set first element (index 0)
                    instructions.push(format!("LD R{} #0", r2));
                    instructions.push(format!("LD R{} #4", r3));
                    instructions.push(format!("MUL.i R{} R{} R{}", r4, r2, r3));
                    instructions.push(format!("ADD.i R{} R{} R{}", r5, r1, r4));
                    instructions.push(format!("ST R{} R{}", r5, r0));
                    
                    // Set second element (index 1)
                    instructions.push(format!("LD R{} #1", r2));
                    instructions.push(format!("LD R{} #4", r3));
                    instructions.push(format!("MUL.i R{} R{} R{}", r4, r2, r3));
                    instructions.push(format!("ADD.i R{} R{} R{}", r5, r1, r4));
                    instructions.push(format!("ST R{} R{}", r5, r0));
                    
                    println!("DEBUG [Codegen]: List assignment instructions generated: {:?}", instructions);
                    symbol_table.insert(var.clone(), 0);
                }
                _ => instructions.push("ERROR".to_string())
            }
        }
        Expr::Boolean(left, op, right) => {
            match (left.as_ref(), right.as_ref()) {
                (Expr::Int(n1), Expr::Variable(var)) | (Expr::Variable(var), Expr::Int(n1)) => {
                    let r0 = reg_alloc.get_next_reg();
                    let r1 = reg_alloc.get_next_reg();
                    let r2 = reg_alloc.get_next_reg();
                    instructions.push(format!("LD R{} {}", r0, n1));
                    instructions.push(format!("LD R{} @{}", r1, var));
                    instructions.push(format!("FL.i R{} R{}", r0, r0));
                    instructions.push(format!("FL.i R{} R{}", r1, r1));
                    match op.as_str() {
                        "!=" => {
                            instructions.push(format!("NE.f R{} R{} R{}", r2, r0, r1));
                            instructions.push(format!("ST @print R{}", r2));
                        },
                        "==" => {
                            instructions.push(format!("EQ.f R{} R{} R{}", r2, r0, r1));
                            instructions.push(format!("ST @print R{}", r2));
                        },
                        _ => instructions.push("ERROR".to_string())
                    }
                },
                _ => instructions.push("ERROR".to_string())
            }
        }
        Expr::ListAccess(var, index) => {
            println!("DEBUG [Codegen]: Processing List access for var: {}", var);
            match index.as_ref() {
                Expr::Int(idx) => {
                    println!("DEBUG [Codegen]: List access index: {}", idx);
                    let r0 = reg_alloc.get_next_reg();
                    let r1 = reg_alloc.get_next_reg();
                    let r2 = reg_alloc.get_next_reg();
                    let r3 = reg_alloc.get_next_reg();
                    let r4 = reg_alloc.get_next_reg();
                    
                    println!("DEBUG [Codegen]: List access registers: r0={}, r1={}, r2={}, r3={}, r4={}", r0, r1, r2, r3, r4);
                    
                    instructions.push(format!("LD R{} @{}", r0, var));
                    instructions.push(format!("LD R{} #{}", r1, idx));
                    instructions.push(format!("LD R{} #4", r2));
                    instructions.push(format!("MUL.i R{} R{} R{}", r3, r1, r2));
                    instructions.push(format!("ADD.i R{} R{} R{}", r4, r0, r3));
                    instructions.push(format!("ST @print R{}", r4));
                    
                    println!("DEBUG [Codegen]: List access instructions generated: {:?}", instructions);
                },
                _ => {
                    println!("DEBUG [Codegen]: Invalid list access index type");
                    instructions.push("ERROR".to_string());
                }
            }
        }
        _ => {
            println!("DEBUG [Codegen]: Unhandled expression type: {:?}", expr);
            instructions.push("ERROR".to_string())
        },
    }

    println!("DEBUG [Codegen]: Generated instructions: {:?}", instructions);
    instructions
}

pub fn generate_assembly(expr: &Expr) -> Vec<String> {
    println!("DEBUG [Codegen]: Starting assembly generation for expr: {:?}", expr);
    let mut result: Vec<String> = Vec::new();
    let mut symbol_table = HashMap::new();
    let mut reg_alloc = RegisterAllocator::new();
    
    let instructions = generate_instructions(expr, &mut symbol_table, &mut reg_alloc);
    
    // If we have an ERROR instruction, add a newline before it
    if instructions.len() == 1 && instructions[0] == "ERROR" {
        if !result.is_empty() && result.last() != Some(&String::new()) {
            result.push(String::new());
        }
        result.push("ERROR".to_string());
    } else {
        result.extend(instructions);
    }
    
    println!("DEBUG [Codegen]: Final assembly: {:?}", result);
    result
}

