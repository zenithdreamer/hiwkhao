use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Value {
    Int(i32),
    Float(f64),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
        }
    }
}

impl Value {
    pub fn as_int(&self) -> i32 {
        match self {
            Value::Int(i) => *i,
            Value::Float(f) => *f as i32,
        }
    }

    pub fn as_float(&self) -> f64 {
        match self {
            Value::Int(i) => *i as f64,
            Value::Float(f) => *f,
        }
    }
}

pub struct VM {
    registers: HashMap<String, Value>,
    memory: Vec<u8>,
    memory_map: HashMap<String, usize>,
    pc: usize,
    program: Vec<String>,
    output: Vec<String>,
    next_addr: usize,
}

impl VM {
    pub fn new(memory_size: usize) -> Self {
        Self {
            registers: HashMap::new(),
            memory: vec![0; memory_size],
            memory_map: HashMap::new(),
            pc: 0,
            program: Vec::new(),
            output: Vec::new(),
            next_addr: 0,
        }
    }

    pub fn load_program(&mut self, program: Vec<String>) {
        self.program = program;
    }

    pub fn step(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return false;
        }

        let instruction = self.program[self.pc].clone();
        self.execute(&instruction);
        self.pc += 1;
        true
    }

    pub fn run(&mut self) {
        while self.step() {}
    }

    fn execute(&mut self, instruction: &str) {
        let parts: Vec<&str> = instruction.split_whitespace().collect();
        match parts[0] {
            "LD" => {
                // LD R1 #123 (immediate)
                // LD R1 @var (memory)
                // LD R1 R2 (register value or memory address)
                let dst_reg = parts[1].to_string();
                let src = parts[2];
                
                if src.starts_with('#') {
                    // Immediate value
                    let value = &src[1..];
                    if value.contains('.') {
                        // Float value
                        let val: f64 = value.parse().unwrap();
                        self.registers.insert(dst_reg, Value::Float(val));
                    } else {
                        // Integer value
                        let val: i32 = value.parse().unwrap();
                        self.registers.insert(dst_reg, Value::Int(val));
                    }
                } else if src.starts_with('@') {
                    // Memory location
                    let var_name = &src[1..];
                    let addr = if let Ok(numeric_addr) = var_name.parse::<usize>() {
                        numeric_addr
                    } else {
                        *self.memory_map.get(var_name).unwrap_or(&0)
                    };
                    
                    if addr + 3 < self.memory.len() {
                        let bytes = [
                            self.memory[addr],
                            self.memory[addr + 1],
                            self.memory[addr + 2],
                            self.memory[addr + 3],
                        ];
                        let value = Value::Int(i32::from_le_bytes(bytes));
                        self.registers.insert(dst_reg, value);
                    }
                } else {
                    // Register value or memory address
                    let src_reg = &src[..];
                    if let Some(value) = self.registers.get(src_reg) {
                        match value {
                            Value::Int(addr) if *addr >= 0 => {
                                // If the register contains a non-negative integer, treat it as a memory address
                                let addr = *addr as usize;
                                if addr + 3 < self.memory.len() {
                                    let bytes = [
                                        self.memory[addr],
                                        self.memory[addr + 1],
                                        self.memory[addr + 2],
                                        self.memory[addr + 3],
                                    ];
                                    let value = Value::Int(i32::from_le_bytes(bytes));
                                    self.registers.insert(dst_reg, value);
                                }
                            },
                            _ => {
                                // Otherwise, just copy the register value
                                self.registers.insert(dst_reg, value.clone());
                            }
                        }
                    }
                }
            }
            "ST" => {
                // ST @var R1
                // ST @print R1
                let dst = parts[1];
                let src_reg = parts[2];
                
                if dst == "@print" {
                    if let Some(value) = self.registers.get(src_reg) {
                        let output = value.to_string();
                        println!("{}", output);
                        self.output.push(output);
                    }
                } else if dst.starts_with('@') {
                    let var_name = &dst[1..];
                    let addr = if let Ok(numeric_addr) = var_name.parse::<usize>() {
                        numeric_addr
                    } else {
                        let addr = self.memory_map.get(var_name).copied().unwrap_or_else(|| {
                            let new_addr = self.next_addr;
                            // Check if the source register contains a list size
                            let size = if let Some(Value::Int(n)) = self.registers.get(src_reg) {
                                (*n as usize) * 4  // Each element needs 4 bytes
                            } else {
                                4  // Default to 4 bytes for single values
                            };
                            self.next_addr += size;
                            // Initialize memory to 0
                            for i in new_addr..new_addr + size {
                                if i < self.memory.len() {
                                    self.memory[i] = 0;
                                }
                            }
                            self.memory_map.insert(var_name.to_string(), new_addr);
                            new_addr
                        });
                        addr
                    };

                    if let Some(value) = self.registers.get(src_reg) {
                        match value {
                            Value::Int(i) => {
                                let bytes = i.to_le_bytes();
                                if addr + bytes.len() <= self.memory.len() {
                                    self.memory[addr..addr + bytes.len()].copy_from_slice(&bytes);
                                }
                            }
                            Value::Float(f) => {
                                let bytes = f.to_le_bytes();
                                if addr + bytes.len() <= self.memory.len() {
                                    self.memory[addr..addr + bytes.len()].copy_from_slice(&bytes);
                                }
                            }
                        }
                    }
                } else {
                    // Store to memory address in register
                    if let Some(dst_reg_value) = self.registers.get(dst) {
                        if let Value::Int(addr) = dst_reg_value {
                            let addr = *addr as usize;
                            if let Some(src_value) = self.registers.get(src_reg) {
                                match src_value {
                                    Value::Int(i) => {
                                        let bytes = i.to_le_bytes();
                                        if addr + bytes.len() <= self.memory.len() {
                                            self.memory[addr..addr + bytes.len()].copy_from_slice(&bytes);
                                        }
                                    }
                                    Value::Float(f) => {
                                        let bytes = f.to_le_bytes();
                                        if addr + bytes.len() <= self.memory.len() {
                                            self.memory[addr..addr + bytes.len()].copy_from_slice(&bytes);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            op if op.starts_with("ADD") => {
                // ADD.i R3 R1 R2
                // ADD.f R3 R1 R2
                let dst_reg = parts[1];
                let src1_reg = parts[2];
                let src2_reg = parts[3];
                
                let r1 = self.registers.get(src1_reg).unwrap();
                let r2 = self.registers.get(src2_reg).unwrap();
                
                let result = if op.ends_with(".f") {
                    Value::Float(r1.as_float() + r2.as_float())
                } else {
                    Value::Int(r1.as_int().wrapping_add(r2.as_int()))
                };
                self.registers.insert(dst_reg.to_string(), result);
            }
            op if op.starts_with("SUB") => {
                // SUB.i R3 R1 R2
                // SUB.f R3 R1 R2
                let dst_reg = parts[1];
                let src1_reg = parts[2];
                let src2_reg = parts[3];
                
                let r1 = self.registers.get(src1_reg).unwrap();
                let r2 = self.registers.get(src2_reg).unwrap();
                
                let result = if op.ends_with(".f") {
                    Value::Float(r1.as_float() - r2.as_float())
                } else {
                    Value::Int(r1.as_int().wrapping_sub(r2.as_int()))
                };
                self.registers.insert(dst_reg.to_string(), result);
            }
            op if op.starts_with("MUL") => {
                // MUL.i R3 R1 R2
                // MUL.f R3 R1 R2
                let dst_reg = parts[1];
                let src1_reg = parts[2];
                let src2_reg = parts[3];
                
                let r1 = self.registers.get(src1_reg).unwrap();
                let r2 = self.registers.get(src2_reg).unwrap();
                
                let result = if op.ends_with(".f") {
                    Value::Float(r1.as_float() * r2.as_float())
                } else {
                    Value::Int(r1.as_int().wrapping_mul(r2.as_int()))
                };
                self.registers.insert(dst_reg.to_string(), result);
            }
            op if op.starts_with("DIV") => {
                // DIV.i R3 R1 R2
                // DIV.f R3 R1 R2
                let dst_reg = parts[1];
                let src1_reg = parts[2];
                let src2_reg = parts[3];
                
                let r1 = self.registers.get(src1_reg).unwrap();
                let r2 = self.registers.get(src2_reg).unwrap();
                
                let result = if op.ends_with(".f") {
                    let divisor = r2.as_float();
                    if divisor == 0.0 { panic!("Division by zero"); }
                    Value::Float(r1.as_float() / divisor)
                } else {
                    let divisor = r2.as_int();
                    if divisor == 0 { panic!("Division by zero"); }
                    Value::Int(r1.as_int().wrapping_div(divisor))
                };
                self.registers.insert(dst_reg.to_string(), result);
            }
            "print" => {
                // print R1
                let reg = parts[1];
                if let Some(value) = self.registers.get(reg) {
                    println!("{}", value);
                }
            }
            "FL.i" => {
                // FL.i R1 R2 (convert int in R2 to float in R1)
                let dst_reg = parts[1].to_string();
                let src_reg = parts[2];
                
                if let Some(value) = self.registers.get(src_reg) {
                    let float_val = match value {
                        Value::Int(i) => Value::Float(*i as f64),
                        Value::Float(f) => Value::Float(*f),
                    };
                    self.registers.insert(dst_reg, float_val);
                }
            }
            "EQ.i" => {
                // EQ.i R3 R1 R2 (R3 = R1 == R2)
                let dst_reg = parts[1].to_string();
                let src1_reg = parts[2];
                let src2_reg = parts[3];
                
                if let (Some(v1), Some(v2)) = (self.registers.get(src1_reg), self.registers.get(src2_reg)) {
                    let result = match (v1, v2) {
                        (Value::Int(i1), Value::Int(i2)) => Value::Int(if i1 == i2 { 1 } else { 0 }),
                        (Value::Float(f1), Value::Float(f2)) => Value::Int(if f1 == f2 { 1 } else { 0 }),
                        _ => Value::Int(0),
                    };
                    self.registers.insert(dst_reg, result);
                }
            }
            "NE.i" => {
                // NE.i R3 R1 R2 (R3 = R1 != R2)
                let dst_reg = parts[1].to_string();
                let src1_reg = parts[2];
                let src2_reg = parts[3];
                
                if let (Some(v1), Some(v2)) = (self.registers.get(src1_reg), self.registers.get(src2_reg)) {
                    let result = match (v1, v2) {
                        (Value::Int(i1), Value::Int(i2)) => Value::Int(if i1 != i2 { 1 } else { 0 }),
                        (Value::Float(f1), Value::Float(f2)) => Value::Int(if f1 != f2 { 1 } else { 0 }),
                        _ => Value::Int(0),
                    };
                    self.registers.insert(dst_reg, result);
                }
            }
            "LT.i" => {
                // LT.i R3 R1 R2 (R3 = R1 < R2)
                let dst_reg = parts[1].to_string();
                let src1_reg = parts[2];
                let src2_reg = parts[3];
                
                if let (Some(v1), Some(v2)) = (self.registers.get(src1_reg), self.registers.get(src2_reg)) {
                    let result = match (v1, v2) {
                        (Value::Int(i1), Value::Int(i2)) => Value::Int(if i1 < i2 { 1 } else { 0 }),
                        (Value::Float(f1), Value::Float(f2)) => Value::Int(if f1 < f2 { 1 } else { 0 }),
                        _ => Value::Int(0),
                    };
                    self.registers.insert(dst_reg, result);
                }
            }
            "GT.i" => {
                // GT.i R3 R1 R2 (R3 = R1 > R2)
                let dst_reg = parts[1].to_string();
                let src1_reg = parts[2];
                let src2_reg = parts[3];
                
                if let (Some(v1), Some(v2)) = (self.registers.get(src1_reg), self.registers.get(src2_reg)) {
                    let result = match (v1, v2) {
                        (Value::Int(i1), Value::Int(i2)) => Value::Int(if i1 > i2 { 1 } else { 0 }),
                        (Value::Float(f1), Value::Float(f2)) => Value::Int(if f1 > f2 { 1 } else { 0 }),
                        _ => Value::Int(0),
                    };
                    self.registers.insert(dst_reg, result);
                }
            }
            "LE.i" => {
                // LE.i R3 R1 R2 (R3 = R1 <= R2)
                let dst_reg = parts[1].to_string();
                let src1_reg = parts[2];
                let src2_reg = parts[3];
                
                if let (Some(v1), Some(v2)) = (self.registers.get(src1_reg), self.registers.get(src2_reg)) {
                    let result = match (v1, v2) {
                        (Value::Int(i1), Value::Int(i2)) => Value::Int(if i1 <= i2 { 1 } else { 0 }),
                        (Value::Float(f1), Value::Float(f2)) => Value::Int(if f1 <= f2 { 1 } else { 0 }),
                        _ => Value::Int(0),
                    };
                    self.registers.insert(dst_reg, result);
                }
            }
            "GE.i" => {
                // GE.i R3 R1 R2 (R3 = R1 >= R2)
                let dst_reg = parts[1].to_string();
                let src1_reg = parts[2];
                let src2_reg = parts[3];
                
                if let (Some(v1), Some(v2)) = (self.registers.get(src1_reg), self.registers.get(src2_reg)) {
                    let result = match (v1, v2) {
                        (Value::Int(i1), Value::Int(i2)) => Value::Int(if i1 >= i2 { 1 } else { 0 }),
                        (Value::Float(f1), Value::Float(f2)) => Value::Int(if f1 >= f2 { 1 } else { 0 }),
                        _ => Value::Int(0),
                    };
                    self.registers.insert(dst_reg, result);
                }
            }
            "EQ.f" => {
                // EQ.f R3 R1 R2 (R3 = R1 == R2, float comparison)
                let dst_reg = parts[1].to_string();
                let src1_reg = parts[2];
                let src2_reg = parts[3];
                
                if let (Some(v1), Some(v2)) = (self.registers.get(src1_reg), self.registers.get(src2_reg)) {
                    let result = match (v1, v2) {
                        (Value::Float(f1), Value::Float(f2)) => Value::Int(if f1 == f2 { 1 } else { 0 }),
                        _ => Value::Int(0),
                    };
                    self.registers.insert(dst_reg, result);
                }
            }
            "NE.f" => {
                // NE.f R3 R1 R2 (R3 = R1 != R2, float comparison)
                let dst_reg = parts[1].to_string();
                let src1_reg = parts[2];
                let src2_reg = parts[3];
                
                if let (Some(v1), Some(v2)) = (self.registers.get(src1_reg), self.registers.get(src2_reg)) {
                    let result = match (v1, v2) {
                        (Value::Float(f1), Value::Float(f2)) => Value::Int(if f1 != f2 { 1 } else { 0 }),
                        _ => Value::Int(0),
                    };
                    self.registers.insert(dst_reg, result);
                }
            }
            "LT.f" => {
                // LT.f R3 R1 R2 (R3 = R1 < R2, float comparison)
                let dst_reg = parts[1].to_string();
                let src1_reg = parts[2];
                let src2_reg = parts[3];
                
                if let (Some(v1), Some(v2)) = (self.registers.get(src1_reg), self.registers.get(src2_reg)) {
                    let result = match (v1, v2) {
                        (Value::Float(f1), Value::Float(f2)) => Value::Int(if f1 < f2 { 1 } else { 0 }),
                        _ => Value::Int(0),
                    };
                    self.registers.insert(dst_reg, result);
                }
            }
            "GT.f" => {
                // GT.f R3 R1 R2 (R3 = R1 > R2, float comparison)
                let dst_reg = parts[1].to_string();
                let src1_reg = parts[2];
                let src2_reg = parts[3];
                
                if let (Some(v1), Some(v2)) = (self.registers.get(src1_reg), self.registers.get(src2_reg)) {
                    let result = match (v1, v2) {
                        (Value::Float(f1), Value::Float(f2)) => Value::Int(if f1 > f2 { 1 } else { 0 }),
                        _ => Value::Int(0),
                    };
                    self.registers.insert(dst_reg, result);
                }
            }
            "LE.f" => {
                // LE.f R3 R1 R2 (R3 = R1 <= R2, float comparison)
                let dst_reg = parts[1].to_string();
                let src1_reg = parts[2];
                let src2_reg = parts[3];
                
                if let (Some(v1), Some(v2)) = (self.registers.get(src1_reg), self.registers.get(src2_reg)) {
                    let result = match (v1, v2) {
                        (Value::Float(f1), Value::Float(f2)) => Value::Int(if f1 <= f2 { 1 } else { 0 }),
                        _ => Value::Int(0),
                    };
                    self.registers.insert(dst_reg, result);
                }
            }
            "GE.f" => {
                // GE.f R3 R1 R2 (R3 = R1 >= R2, float comparison)
                let dst_reg = parts[1].to_string();
                let src1_reg = parts[2];
                let src2_reg = parts[3];
                
                if let (Some(v1), Some(v2)) = (self.registers.get(src1_reg), self.registers.get(src2_reg)) {
                    let result = match (v1, v2) {
                        (Value::Float(f1), Value::Float(f2)) => Value::Int(if f1 >= f2 { 1 } else { 0 }),
                        _ => Value::Int(0),
                    };
                    self.registers.insert(dst_reg, result);
                }
            }
            "NEG.i" => {
                // NEG.i R1 R2 (R1 = -R2)
                let dst_reg = parts[1].to_string();
                let src_reg = parts[2];
                
                if let Some(value) = self.registers.get(src_reg) {
                    let result = match value {
                        Value::Int(i) => Value::Int(-*i),
                        Value::Float(f) => Value::Int(-(*f as i32)),
                    };
                    self.registers.insert(dst_reg, result);
                }
            }
            "NEG.f" => {
                // NEG.f R1 R2 (R1 = -R2)
                let dst_reg = parts[1].to_string();
                let src_reg = parts[2];
                
                if let Some(value) = self.registers.get(src_reg) {
                    let result = match value {
                        Value::Int(i) => Value::Float(-(*i as f64)),
                        Value::Float(f) => Value::Float(-*f),
                    };
                    self.registers.insert(dst_reg, result);
                }
            }
            _ => {
                panic!("Unknown instruction: {}", parts[0]);
            }
        }
    }

    fn load_from_memory(&self, addr: usize) -> Value {
        if addr + 3 >= self.memory.len() {
            panic!("Memory access out of bounds");
        }
        let bytes = [
            self.memory[addr],
            self.memory[addr + 1],
            self.memory[addr + 2],
            self.memory[addr + 3],
        ];
        Value::Int(i32::from_le_bytes(bytes))
    }

    fn store_to_memory(&mut self, addr: usize, value: &Value) {
        if addr + 3 >= self.memory.len() {
            panic!("Memory access out of bounds");
        }
        let bytes = match value {
            Value::Int(i) => i.to_le_bytes(),
            Value::Float(f) => (*f as i32).to_le_bytes(),
        };
        self.memory[addr..addr + 4].copy_from_slice(&bytes);
    }

    pub fn get_state(&self) -> (&HashMap<String, Value>, &[u8], usize) {
        (&self.registers, &self.memory, self.pc)
    }

    pub fn get_program(&self) -> Vec<String> {
        self.program.clone()
    }

    pub fn get_output(&self) -> &[String] {
        &self.output
    }

    pub fn clear_output(&mut self) {
        self.output.clear();
    }
}
