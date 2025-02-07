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
    
    // Helper function to get the value of a unary operation
    fn get_unary_value(expr: &Expr) -> Option<(bool, Expr)> {
        match expr {
            Expr::UnaryOp(op, value) => {
                match (op.as_str(), value.as_ref()) {
                    ("-", value @ (Expr::Int(_) | Expr::Float(_))) => Some((true, value.clone())),
                    _ => None
                }
            }
            value @ (Expr::Int(_) | Expr::Float(_)) => Some((false, value.clone())),
            _ => None
        }
    }
    
    match (left, right) {
        // Handle cases where either operand might be a unary operation
        (left, right) if get_unary_value(left).is_some() && get_unary_value(right).is_some() => {
            let (is_neg1, val1) = get_unary_value(left).unwrap();
            let (is_neg2, val2) = get_unary_value(right).unwrap();
            
            // Check for division by zero first
            if op == "/" {
                match val2 {
                    Expr::Int(n) => if (if is_neg2 { -n } else { n }) == 0 {
                        instructions.push("ERROR".to_string());
                        return instructions;
                    },
                    Expr::Float(n) => if (if is_neg2 { -n } else { n }) == 0.0 {
                        instructions.push("ERROR".to_string());
                        return instructions;
                    },
                    _ => unreachable!()
                }
            }
            
            let r0 = reg_alloc.get_next_reg();
            let r1 = reg_alloc.get_next_reg();
            let r2 = reg_alloc.get_next_reg();
            
            // Determine if we need float operations
            let is_float = matches!(val1, Expr::Float(_)) || matches!(val2, Expr::Float(_));
            
            // Load first value
            match val1 {
                Expr::Int(n) => {
                    let n = if is_neg1 { -n } else { n };
                    instructions.push(format!("LD R{} #{}", r0, n));
                    if is_float {
                        instructions.push(format!("FL.i R{} R{}", r0, r0));
                    }
                }
                Expr::Float(n) => {
                    let n = if is_neg1 { -n } else { n };
                    instructions.push(format!("LD R{} #{}", r0, n));
                }
                _ => unreachable!()
            }
            
            // Load second value
            match val2 {
                Expr::Int(n) => {
                    let n = if is_neg2 { -n } else { n };
                    instructions.push(format!("LD R{} #{}", r1, n));
                    if is_float {
                        instructions.push(format!("FL.i R{} R{}", r1, r1));
                    }
                }
                Expr::Float(n) => {
                    let n = if is_neg2 { -n } else { n };
                    instructions.push(format!("LD R{} #{}", r1, n));
                }
                _ => unreachable!()
            }
            
            let op_code = match (op, is_float) {
                ("+", false) => "ADD.i",
                ("-", false) => "SUB.i",
                ("*", false) => "MUL.i",
                ("/", false) => "DIV.i",
                ("+", true) => "ADD.f",
                ("-", true) => "SUB.f",
                ("*", true) => "MUL.f",
                ("/", true) => "DIV.f",
                ("==", _) => if is_float { "EQ.f" } else { "EQ.i" },
                ("!=", _) => if is_float { "NE.f" } else { "NE.i" },
                ("<", _) => if is_float { "LT.f" } else { "LT.i" },
                (">", _) => if is_float { "GT.f" } else { "GT.i" },
                ("<=", _) => if is_float { "LE.f" } else { "LE.i" },
                (">=", _) => if is_float { "GE.f" } else { "GE.i" },
                _ => {
                    instructions.push("ERROR".to_string());
                    return instructions;
                }
            };
            
            instructions.push(format!("{} R{} R{} R{}", op_code, r2, r0, r1));
            instructions.push(format!("ST @print R{}", r2));
        }
        (Expr::Int(n1), Expr::Int(n2)) => {
            // Check for division by zero first
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
            instructions.push(format!("LD R{} #{}.0", r1, n2));
            
            let op_code = match op {
                "+" => "ADD.f",
                "-" => "SUB.f",
                "*" => "MUL.f",
                "/" => "DIV.f",
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
            
            // Load the constant first, then the variable
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
        (Expr::Float(n1), Expr::Float(n2)) => {
            let r0 = reg_alloc.get_next_reg();
            let r1 = reg_alloc.get_next_reg();
            let r2 = reg_alloc.get_next_reg();
            
            instructions.push(format!("LD R{} #{}.0", r0, n1));
            instructions.push(format!("LD R{} #{}.0", r1, n2));
            
            let op_code = match op {
                "+" => "ADD.f",
                "-" => "SUB.f",
                "*" => "MUL.f",
                "/" => "DIV.f",
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
            // Check if this is a list element assignment
            if var.contains('[') && var.contains(']') {
                let parts: Vec<&str> = var.split('[').collect();
                let list_name = parts[0];
                let index = parts[1].trim_end_matches(']').parse::<i64>().unwrap();
                
                match value.as_ref() {
                    Expr::Int(n) => {
                        let r0 = reg_alloc.get_next_reg();
                        let r1 = reg_alloc.get_next_reg();
                        let r2 = reg_alloc.get_next_reg();
                        let r3 = reg_alloc.get_next_reg();
                        let r4 = reg_alloc.get_next_reg();
                        
                        instructions.push(format!("LD R{} #{}", r0, n));
                        instructions.push(format!("LD R{} @{}", r1, list_name));
                        instructions.push(format!("LD R{} #{}", r2, index));
                        instructions.push(format!("LD R{} #4", r3));
                        instructions.push(format!("MUL.i R{} R{} R{}", r4, r2, r3));
                        instructions.push(format!("ADD.i R{} R{} R{}", r2, r1, r4));
                        instructions.push(format!("ST R{} R{}", r2, r0));
                    }
                    Expr::Float(n) => {
                        let r0 = reg_alloc.get_next_reg();
                        let r1 = reg_alloc.get_next_reg();
                        let r2 = reg_alloc.get_next_reg();
                        let r3 = reg_alloc.get_next_reg();
                        let r4 = reg_alloc.get_next_reg();
                        
                        instructions.push(format!("LD R{} #{}", r0, n));
                        instructions.push(format!("LD R{} @{}", r1, list_name));
                        instructions.push(format!("LD R{} #{}", r2, index));
                        instructions.push(format!("LD R{} #4", r3));
                        instructions.push(format!("MUL.i R{} R{} R{}", r4, r2, r3));
                        instructions.push(format!("ADD.i R{} R{} R{}", r2, r1, r4));
                        instructions.push(format!("ST R{} R{}", r2, r0));
                    }
                    Expr::UnaryOp(op, expr) => {
                        match (op.as_str(), expr.as_ref()) {
                            ("-", Expr::Int(n)) => {
                                let r0 = reg_alloc.get_next_reg();
                                let r1 = reg_alloc.get_next_reg();
                                let r2 = reg_alloc.get_next_reg();
                                let r3 = reg_alloc.get_next_reg();
                                let r4 = reg_alloc.get_next_reg();
                                
                                instructions.push(format!("LD R{} #{}", r0, -n));
                                instructions.push(format!("LD R{} @{}", r1, list_name));
                                instructions.push(format!("LD R{} #{}", r2, index));
                                instructions.push(format!("LD R{} #4", r3));
                                instructions.push(format!("MUL.i R{} R{} R{}", r4, r2, r3));
                                instructions.push(format!("ADD.i R{} R{} R{}", r2, r1, r4));
                                instructions.push(format!("ST R{} R{}", r2, r0));
                            }
                            ("-", Expr::Float(n)) => {
                                let r0 = reg_alloc.get_next_reg();
                                let r1 = reg_alloc.get_next_reg();
                                let r2 = reg_alloc.get_next_reg();
                                let r3 = reg_alloc.get_next_reg();
                                let r4 = reg_alloc.get_next_reg();
                                
                                instructions.push(format!("LD R{} #{}", r0, -n));
                                instructions.push(format!("LD R{} @{}", r1, list_name));
                                instructions.push(format!("LD R{} #{}", r2, index));
                                instructions.push(format!("LD R{} #4", r3));
                                instructions.push(format!("MUL.i R{} R{} R{}", r4, r2, r3));
                                instructions.push(format!("ADD.i R{} R{} R{}", r2, r1, r4));
                                instructions.push(format!("ST R{} R{}", r2, r0));
                            }
                            _ => instructions.push("ERROR".to_string())
                        }
                    }
                    _ => instructions.push("ERROR".to_string())
                }
            } else {
                match value.as_ref() {
                    Expr::Int(n) => {
                        let r0 = reg_alloc.get_next_reg();
                        instructions.push(format!("LD R{} #{}", r0, n));
                        instructions.push(format!("ST @{} R{}", var, r0));
                        symbol_table.insert(var.clone(), *n);
                    }
                    Expr::Float(n) => {
                        let r0 = reg_alloc.get_next_reg();
                        instructions.push(format!("LD R{} #{}", r0, n));
                        instructions.push(format!("ST @{} R{}", var, r0));
                        symbol_table.insert(var.clone(), n.to_bits() as i64);
                    }
                    Expr::Variable(name) => {
                        let r0 = reg_alloc.get_next_reg();
                        instructions.push(format!("LD R{} @{}", r0, name));
                        instructions.push(format!("ST @{} R{}", var, r0));
                    }
                    Expr::ListAccess(list_name, index) => {
                        match index.as_ref() {
                            Expr::Int(idx) => {
                                let r0 = reg_alloc.get_next_reg();
                                let r1 = reg_alloc.get_next_reg();
                                let r2 = reg_alloc.get_next_reg();
                                let r3 = reg_alloc.get_next_reg();
                                let r4 = reg_alloc.get_next_reg();
                                
                                instructions.push(format!("LD R{} @{}", r0, list_name));
                                instructions.push(format!("LD R{} #{}", r1, idx));
                                instructions.push(format!("LD R{} #4", r2));
                                instructions.push(format!("MUL.i R{} R{} R{}", r3, r1, r2));
                                instructions.push(format!("ADD.i R{} R{} R{}", r4, r0, r3));
                                instructions.push(format!("LD R0 R{}", r4));
                                instructions.push(format!("ST @{} R0", var));
                            }
                            _ => instructions.push("ERROR".to_string())
                        }
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
        }
        Expr::Boolean(left, op, right) => {
            let r0 = reg_alloc.get_next_reg();
            let r1 = reg_alloc.get_next_reg();
            let r2 = reg_alloc.get_next_reg();

            // Load operands in the correct order (constants first, then variables)
            match (left.as_ref(), right.as_ref()) {
                (Expr::Variable(var), Expr::Int(n)) => {
                    if op == "!=" {
                        instructions.push(format!("LD R{} #{}", r0, n));
                        instructions.push(format!("LD R{} @{}", r1, var));
                        instructions.push(format!("FL.i R{} R{}", r0, r0));
                        instructions.push(format!("FL.i R{} R{}", r1, r1));
                        instructions.push(format!("NE.f R{} R{} R{}", r2, r0, r1));
                        instructions.push(format!("ST @print R{}", r2));
                        return instructions;
                    } else {
                        instructions.push(format!("LD R{} #{}", r0, n));
                        instructions.push(format!("LD R{} @{}", r1, var));
                    }
                }
                (Expr::Int(n), Expr::Variable(var)) => {
                    instructions.push(format!("LD R{} #{}", r0, n));
                    instructions.push(format!("LD R{} @{}", r1, var));
                }
                (Expr::Variable(var1), Expr::Variable(var2)) => {
                    instructions.push(format!("LD R{} @{}", r0, var1));
                    instructions.push(format!("LD R{} @{}", r1, var2));
                }
                (Expr::Int(n1), Expr::Int(n2)) => {
                    instructions.push(format!("LD R{} #{}", r0, n1));
                    instructions.push(format!("LD R{} #{}", r1, n2));
                }
                _ => {
                    instructions.push("ERROR".to_string());
                    return instructions;
                }
            }

            // Generate comparison instruction
            let op_code = match op.as_str() {
                ">" => "GT.i",
                "<" => "LT.i",
                ">=" => "GE.i",
                "<=" => "LE.i",
                "==" => "EQ.i",
                "!=" => "NE.i",
                _ => {
                    instructions.push("ERROR".to_string());
                    return instructions;
                }
            };

            // For variable-constant comparisons, swap the operands in the instruction
            match (left.as_ref(), right.as_ref()) {
                (Expr::Variable(_), Expr::Int(_)) => {
                    instructions.push(format!("{} R{} R{} R{}", op_code, r2, r1, r0));
                }
                _ => {
                    instructions.push(format!("{} R{} R{} R{}", op_code, r2, r0, r1));
                }
            }
            instructions.push(format!("ST @print R{}", r2));
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


