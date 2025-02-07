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
                    if n.fract() == 0.0 {
                        instructions.push(format!("LD R{} #{}.0", r0, n));
                    } else {
                        instructions.push(format!("LD R{} #{}", r0, n));
                    }
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
                    if n.fract() == 0.0 {
                        instructions.push(format!("LD R{} #{}.0", r1, n));
                    } else {
                        instructions.push(format!("LD R{} #{}", r1, n));
                    }
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
        (Expr::Variable(var1), Expr::Variable(var2)) => {
            let r0 = reg_alloc.get_next_reg();
            let r1 = reg_alloc.get_next_reg();
            let r2 = reg_alloc.get_next_reg();
            
            instructions.push(format!("LD R{} @{}", r0, var1));
            instructions.push(format!("LD R{} @{}", r1, var2));
            
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

fn generate_instructions(
    expr: &Expr,
    reg_alloc: &mut RegisterAllocator,
    symbol_table: &mut HashMap<String, i64>,
    instructions: &mut Vec<String>
) {
    println!("DEBUG [Codegen]: Starting instruction generation for expr: {:?}", expr);
    let mut temp_instructions = Vec::new();
    match expr {
        Expr::Assignment(var, expr) => {
            // Check if this is a list element assignment
            if var.contains('[') && var.contains(']') {
                let parts: Vec<&str> = var.split('[').collect();
                let list_name = parts[0];
                let index = parts[1].trim_end_matches(']').parse::<i64>().unwrap();
                
                match expr.as_ref() {
                    Expr::Int(n) => {
                        let r0 = reg_alloc.get_next_reg();
                        let r1 = reg_alloc.get_next_reg();
                        let r2 = reg_alloc.get_next_reg();
                        let r3 = reg_alloc.get_next_reg();
                        let r4 = reg_alloc.get_next_reg();
                        
                        temp_instructions.push(format!("LD R{} #{}", r0, n));
                        temp_instructions.push(format!("LD R{} @{}", r1, list_name));
                        temp_instructions.push(format!("LD R{} #{}", r2, index));
                        temp_instructions.push(format!("LD R{} #4", r3));
                        temp_instructions.push(format!("MUL.i R{} R{} R{}", r4, r2, r3));
                        temp_instructions.push(format!("ADD.i R{} R{} R{}", r2, r1, r4));
                        temp_instructions.push(format!("ST R{} R{}", r2, r0));
                    }
                    Expr::Float(n) => {
                        let r0 = reg_alloc.get_next_reg();
                        let r1 = reg_alloc.get_next_reg();
                        let r2 = reg_alloc.get_next_reg();
                        let r3 = reg_alloc.get_next_reg();
                        let r4 = reg_alloc.get_next_reg();
                        
                        temp_instructions.push(format!("LD R{} #{}", r0, n));
                        temp_instructions.push(format!("LD R{} @{}", r1, list_name));
                        temp_instructions.push(format!("LD R{} #{}", r2, index));
                        temp_instructions.push(format!("LD R{} #4", r3));
                        temp_instructions.push(format!("MUL.i R{} R{} R{}", r4, r2, r3));
                        temp_instructions.push(format!("ADD.i R{} R{} R{}", r2, r1, r4));
                        temp_instructions.push(format!("ST R{} R{}", r2, r0));
                    }
                    Expr::ListAccess(src_list_name, idx) => {
                        if let Expr::Int(src_index) = idx.as_ref() {
                            let r0 = reg_alloc.get_next_reg();
                            let r1 = reg_alloc.get_next_reg();
                            let r2 = reg_alloc.get_next_reg();
                            let r3 = reg_alloc.get_next_reg();
                            let r4 = reg_alloc.get_next_reg();
                            let r5 = reg_alloc.get_next_reg();
                            
                            // Load source list element
                            temp_instructions.push(format!("LD R{} @{}", r0, src_list_name));
                            temp_instructions.push(format!("LD R{} #{}", r1, src_index));
                            temp_instructions.push(format!("LD R{} #4", r2));
                            temp_instructions.push(format!("MUL.i R{} R{} R{}", r3, r1, r2));
                            temp_instructions.push(format!("ADD.i R{} R{} R{}", r4, r0, r3));
                            temp_instructions.push(format!("LD R{} R{}", r5, r4));
                            
                            // Store into target list element
                            temp_instructions.push(format!("LD R{} @{}", r0, list_name));
                            temp_instructions.push(format!("LD R{} #{}", r1, index));
                            temp_instructions.push(format!("LD R{} #4", r2));
                            temp_instructions.push(format!("MUL.i R{} R{} R{}", r3, r1, r2));
                            temp_instructions.push(format!("ADD.i R{} R{} R{}", r4, r0, r3));
                            temp_instructions.push(format!("ST R{} R{}", r4, r5));
                        } else {
                            temp_instructions.push("ERROR".to_string());
                        }
                    }
                    Expr::UnaryOp(op, expr) => {
                        match (op.as_str(), expr.as_ref()) {
                            ("-", Expr::Int(n)) => {
                                let r0 = reg_alloc.get_next_reg();
                                let r1 = reg_alloc.get_next_reg();
                                let r2 = reg_alloc.get_next_reg();
                                let r3 = reg_alloc.get_next_reg();
                                let r4 = reg_alloc.get_next_reg();
                                
                                temp_instructions.push(format!("LD R{} #{}", r0, -n));
                                temp_instructions.push(format!("LD R{} @{}", r1, list_name));
                                temp_instructions.push(format!("LD R{} #{}", r2, index));
                                temp_instructions.push(format!("LD R{} #4", r3));
                                temp_instructions.push(format!("MUL.i R{} R{} R{}", r4, r2, r3));
                                temp_instructions.push(format!("ADD.i R{} R{} R{}", r2, r1, r4));
                                temp_instructions.push(format!("ST R{} R{}", r2, r0));
                            }
                            ("-", Expr::Float(n)) => {
                                let r0 = reg_alloc.get_next_reg();
                                let r1 = reg_alloc.get_next_reg();
                                let r2 = reg_alloc.get_next_reg();
                                let r3 = reg_alloc.get_next_reg();
                                let r4 = reg_alloc.get_next_reg();
                                
                                temp_instructions.push(format!("LD R{} #{}", r0, -n));
                                temp_instructions.push(format!("LD R{} @{}", r1, list_name));
                                temp_instructions.push(format!("LD R{} #{}", r2, index));
                                temp_instructions.push(format!("LD R{} #4", r3));
                                temp_instructions.push(format!("MUL.i R{} R{} R{}", r4, r2, r3));
                                temp_instructions.push(format!("ADD.i R{} R{} R{}", r2, r1, r4));
                                temp_instructions.push(format!("ST R{} R{}", r2, r0));
                            }
                            _ => temp_instructions.push("ERROR".to_string())
                        }
                    }
                    _ => temp_instructions.push("ERROR".to_string())
                }
            } else {
                match expr.as_ref() {
                    Expr::Int(n) => {
                        let r0 = reg_alloc.get_next_reg();
                        temp_instructions.push(format!("LD R{} #{}", r0, n));
                        temp_instructions.push(format!("ST @{} R{}", var, r0));
                        symbol_table.insert(var.clone(), *n);
                    }
                    Expr::Float(n) => {
                        let r0 = reg_alloc.get_next_reg();
                        temp_instructions.push(format!("LD R{} #{}", r0, n));
                        temp_instructions.push(format!("ST @{} R{}", var, r0));
                        symbol_table.insert(var.clone(), n.to_bits() as i64);
                    }
                    Expr::Variable(name) => {
                        let r0 = reg_alloc.get_next_reg();
                        temp_instructions.push(format!("LD R{} @{}", r0, name));
                        temp_instructions.push(format!("ST @{} R{}", var, r0));
                    }
                    Expr::List(_) => {
                        let r0 = reg_alloc.get_next_reg();
                        let r1 = reg_alloc.get_next_reg();
                        let r2 = reg_alloc.get_next_reg();
                        let r3 = reg_alloc.get_next_reg();
                        let r4 = reg_alloc.get_next_reg();
                        let r5 = reg_alloc.get_next_reg();
                        
                        temp_instructions.push(format!("LD R{} #0", r0));
                        temp_instructions.push(format!("LD R{} @{}", r1, var));
                        temp_instructions.push(format!("LD R{} #0", r2));
                        temp_instructions.push(format!("LD R{} #4", r3));
                        temp_instructions.push(format!("MUL.i R{} R{} R{}", r4, r2, r3));
                        temp_instructions.push(format!("ADD.i R{} R{} R{}", r5, r1, r4));
                        temp_instructions.push(format!("ST R{} R{}", r5, r0));
                        
                        temp_instructions.push(format!("LD R{} #1", r2));
                        temp_instructions.push(format!("LD R{} #4", r3));
                        temp_instructions.push(format!("MUL.i R{} R{} R{}", r4, r2, r3));
                        temp_instructions.push(format!("ADD.i R{} R{} R{}", r5, r1, r4));
                        temp_instructions.push(format!("ST R{} R{}", r5, r0));
                        
                        println!("DEBUG [Codegen]: List assignment instructions generated: {:?}", temp_instructions);
                        symbol_table.insert(var.clone(), 0);
                    }
                    Expr::ListAccess(list_name, idx) => {
                        if let Expr::Int(index) = idx.as_ref() {
                            let r0 = reg_alloc.get_next_reg();
                            let r1 = reg_alloc.get_next_reg();
                            let r2 = reg_alloc.get_next_reg();
                            let r3 = reg_alloc.get_next_reg();
                            let r4 = reg_alloc.get_next_reg();
                            
                            temp_instructions.push(format!("LD R{} @{}", r0, list_name));
                            temp_instructions.push(format!("LD R{} #{}", r1, index));
                            temp_instructions.push(format!("LD R{} #4", r2));
                            temp_instructions.push(format!("MUL.i R{} R{} R{}", r3, r1, r2));
                            temp_instructions.push(format!("ADD.i R{} R{} R{}", r4, r0, r3));
                            temp_instructions.push(format!("LD R{} R{}", r0, r4));
                            temp_instructions.push(format!("ST @{} R{}", var, r0));
                        } else {
                            temp_instructions.push("ERROR".to_string());
                        }
                    }
                    _ => temp_instructions.push("ERROR".to_string())
                }
            }
        }
        Expr::Int(n) => {
            println!("DEBUG [Codegen]: Generating instructions for integer: {}", n);
            let r0 = reg_alloc.get_next_reg();
            temp_instructions.push(format!("LD R{} #{}", r0, n));
            temp_instructions.push(format!("ST @print R{}", r0));
        }
        Expr::Float(n) => {
            println!("DEBUG [Codegen]: Generating instructions for float: {}", n);
            let r0 = reg_alloc.get_next_reg();
            temp_instructions.push(format!("LD R{} #{}", r0, n));
            temp_instructions.push(format!("ST @print R{}", r0));
        }
        Expr::BinaryOp(left, op, right) => {
            println!("DEBUG [Codegen]: Generating instructions for binary op: {} {:?} {:?}", op, left, right);
            match op.as_str() {
                "+" | "-" | "*" | "/" | "==" | "!=" | "<" | ">" | "<=" | ">=" => {
                    println!("DEBUG [Codegen]: {} operation", match op.as_str() {
                        "+" => "Addition",
                        "-" => "Subtraction",
                        "*" => "Multiplication",
                        "/" => "Division",
                        "==" => "Equality",
                        "!=" => "Inequality",
                        "<" => "Less than",
                        ">" => "Greater than",
                        "<=" => "Less than or equal",
                        ">=" => "Greater than or equal",
                        _ => unreachable!()
                    });
                    temp_instructions.extend(generate_binary_arithmetic(left, right, op, symbol_table, reg_alloc))
                },
                "^" | "POW" => {
                    println!("DEBUG [Codegen]: Power operation detected");
                    temp_instructions.push("ERROR".to_string())
                }
                _ => {
                    println!("DEBUG [Codegen]: Unknown binary operator: {}", op);
                    temp_instructions.push("ERROR".to_string())
                },
            }
        }
        Expr::Boolean(left, op, right) => {
            let r0 = reg_alloc.get_next_reg();
            let r1 = reg_alloc.get_next_reg();
            let r2 = reg_alloc.get_next_reg();

            // Helper function to check if an expression is a float or negative float
            fn is_float_expr(expr: &Expr) -> bool {
                match expr {
                    Expr::Float(_) => true,
                    Expr::UnaryOp(op, inner) if op == "-" => matches!(inner.as_ref(), Expr::Float(_)),
                    _ => false
                }
            }

            // Determine if we need float operations
            let needs_float = is_float_expr(left) || is_float_expr(right) || op == "!=";

            // Load the operands in the correct order
            match (left.as_ref(), right.as_ref()) {
                (Expr::Variable(var_name), Expr::Int(val)) => {
                    if needs_float {
                        temp_instructions.push(format!("LD R0 @{}", var_name));
                        temp_instructions.push(format!("LD R1 #{}", val));
                        temp_instructions.push(format!("FL.i R0 R0"));
                        temp_instructions.push(format!("FL.i R1 R1"));
                    } else {
                        temp_instructions.push(format!("LD R0 @{}", var_name));
                        temp_instructions.push(format!("LD R1 #{}", val));
                    }
                },
                (Expr::Int(val), Expr::Variable(var_name)) => {
                    if needs_float {
                        temp_instructions.push(format!("LD R0 #{}", val));
                        temp_instructions.push(format!("LD R1 @{}", var_name));
                        temp_instructions.push(format!("FL.i R0 R0"));
                        temp_instructions.push(format!("FL.i R1 R1"));
                    } else {
                        temp_instructions.push(format!("LD R0 #{}", val));
                        temp_instructions.push(format!("LD R1 @{}", var_name));
                    }
                },
                (Expr::Variable(var_name1), Expr::Variable(var_name2)) => {
                    temp_instructions.push(format!("LD R0 @{}", var_name1));
                    temp_instructions.push(format!("LD R1 @{}", var_name2));
                    if needs_float {
                        temp_instructions.push(format!("FL.i R0 R0"));
                        temp_instructions.push(format!("FL.i R1 R1"));
                    }
                },
                (Expr::Int(val), Expr::Float(float_val)) => {
                    temp_instructions.push(format!("LD R0 #{}", val));
                    temp_instructions.push(format!("FL.i R0 R0"));
                    temp_instructions.push(format!("LD R1 #{}", float_val));
                },
                (Expr::Float(float_val), Expr::Int(val)) => {
                    temp_instructions.push(format!("LD R0 #{}", float_val));
                    temp_instructions.push(format!("LD R1 #{}", val));
                    temp_instructions.push(format!("FL.i R1 R1"));
                },
                (Expr::Variable(var_name), Expr::Float(val)) => {
                    temp_instructions.push(format!("LD R0 @{}", var_name));
                    temp_instructions.push(format!("FL.i R0 R0"));
                    temp_instructions.push(format!("LD R1 #{}", val));
                },
                (Expr::Float(val), Expr::Variable(var_name)) => {
                    temp_instructions.push(format!("LD R0 #{}", val));
                    temp_instructions.push(format!("LD R1 @{}", var_name));
                    temp_instructions.push(format!("FL.i R1 R1"));
                },
                (Expr::UnaryOp(op, inner), right) if op == "-" => {
                    match inner.as_ref() {
                        Expr::Variable(var) => {
                            temp_instructions.push(format!("LD R0 @{}", var));
                            temp_instructions.push(format!("NEG.i R0 R0"));
                        }
                        Expr::Float(n) => {
                            temp_instructions.push(format!("LD R0 #{}", -n));
                        }
                        Expr::Int(n) => {
                            temp_instructions.push(format!("LD R0 #{}", -n));
                        }
                        _ => {
                            temp_instructions.push("ERROR".to_string());
                            return;
                        }
                    }
                    match right {
                        Expr::Variable(var) => {
                            temp_instructions.push(format!("LD R1 @{}", var));
                        }
                        Expr::Int(n) => {
                            temp_instructions.push(format!("LD R1 #{}", n));
                        }
                        Expr::Float(n) => {
                            temp_instructions.push(format!("LD R1 #{}", n));
                        }
                        _ => {
                            temp_instructions.push("ERROR".to_string());
                            return;
                        }
                    }
                    if needs_float {
                        if !is_float_expr(inner) {
                            temp_instructions.push(format!("FL.i R0 R0"));
                        }
                        if !is_float_expr(right) {
                            temp_instructions.push(format!("FL.i R1 R1"));
                        }
                    }
                }
                _ => {
                    temp_instructions.push("ERROR".to_string());
                    return;
                }
            };

            // Generate comparison instruction with correct operand order per ILOC spec
            let op_code = match op.as_str() {
                ">" => if needs_float { "GT.f" } else { "GT.i" },
                "<" => if needs_float { "LT.f" } else { "LT.i" },
                ">=" => if needs_float { "GE.f" } else { "GE.i" },
                "<=" => if needs_float { "LE.f" } else { "LE.i" },
                "==" => if needs_float { "EQ.f" } else { "EQ.i" },
                "!=" => if needs_float { "NE.f" } else { "NE.i" },
                _ => {
                    temp_instructions.push("ERROR".to_string());
                    return;
                }
            };
            temp_instructions.push(format!("{} R{} R{} R{}", op_code, r2, r0, r1));
            temp_instructions.push(format!("ST @print R{}", r2));
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
                    
                    temp_instructions.push(format!("LD R{} @{}", r0, var));
                    temp_instructions.push(format!("LD R{} #{}", r1, idx));
                    temp_instructions.push(format!("LD R{} #4", r2));
                    temp_instructions.push(format!("MUL.i R{} R{} R{}", r3, r1, r2));
                    temp_instructions.push(format!("ADD.i R{} R{} R{}", r4, r0, r3));
                    temp_instructions.push(format!("ST @print R{}", r4));
                    
                    println!("DEBUG [Codegen]: List access instructions generated: {:?}", temp_instructions);
                },
                _ => {
                    println!("DEBUG [Codegen]: Invalid list access index type");
                    temp_instructions.push("ERROR".to_string());
                }
            }
        }
        _ => {
            println!("DEBUG [Codegen]: Unhandled expression type: {:?}", expr);
            temp_instructions.push("ERROR".to_string())
        },
    }
    instructions.extend(temp_instructions);
}

pub fn generate_assembly(expr: &Expr) -> Vec<String> {
    println!("DEBUG [Codegen]: Starting assembly generation for expr: {:?}", expr);
    let mut reg_alloc = RegisterAllocator::new();
    let mut symbol_table = HashMap::new();
    let mut instructions = Vec::new();
    generate_instructions(expr, &mut reg_alloc, &mut symbol_table, &mut instructions);
    println!("DEBUG [Codegen]: Final assembly: {:?}", instructions);
    instructions
}

#[test]
fn test_undefined_variable() {
    let expr = Expr::Assignment(
        String::from("x"),
        Box::new(Expr::Variable(String::from("y")))
    );
    let expected = vec![
        "LD R0 @y",
        "ST @x R0"
    ];
    assert_eq!(generate_assembly(&expr), expected);
}

#[test]
fn test_variable_arithmetic() {
    // Test variable addition
    let expr = Expr::BinaryOp(
        Box::new(Expr::Variable(String::from("x"))),
        String::from("+"),
        Box::new(Expr::Variable(String::from("y")))
    );
    let expected = vec![
        "LD R0 @x",
        "LD R1 @y",
        "ADD.i R2 R0 R1",
        "ST @print R2"
    ];
    assert_eq!(generate_assembly(&expr), expected);

    // Test variable subtraction
    let expr = Expr::BinaryOp(
        Box::new(Expr::Variable(String::from("a"))),
        String::from("-"),
        Box::new(Expr::Variable(String::from("b")))
    );
    let expected = vec![
        "LD R0 @a",
        "LD R1 @b",
        "SUB.i R2 R0 R1",
        "ST @print R2"
    ];
    assert_eq!(generate_assembly(&expr), expected);

    // Test variable multiplication
    let expr = Expr::BinaryOp(
        Box::new(Expr::Variable(String::from("x"))),
        String::from("*"),
        Box::new(Expr::Variable(String::from("y")))
    );
    let expected = vec![
        "LD R0 @x",
        "LD R1 @y",
        "MUL.i R2 R0 R1",
        "ST @print R2"
    ];
    assert_eq!(generate_assembly(&expr), expected);
}

#[test]
fn test_variable_comparison() {
    // Test variable equality
    let expr = Expr::BinaryOp(
        Box::new(Expr::Variable(String::from("x"))),
        String::from("=="),
        Box::new(Expr::Variable(String::from("y")))
    );
    let expected = vec![
        "LD R0 @x",
        "LD R1 @y",
        "EQ.i R2 R0 R1",
        "ST @print R2"
    ];
    assert_eq!(generate_assembly(&expr), expected);

    // Test variable less than
    let expr = Expr::BinaryOp(
        Box::new(Expr::Variable(String::from("x"))),
        String::from("<"),
        Box::new(Expr::Variable(String::from("y")))
    );
    let expected = vec![
        "LD R0 @x",
        "LD R1 @y",
        "LT.i R2 R0 R1",
        "ST @print R2"
    ];
    assert_eq!(generate_assembly(&expr), expected);
}


