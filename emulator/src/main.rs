use crate::vm::VM;
use crate::parser::parse_iloc;
use std::sync::{Arc, Mutex};

mod parser;
mod gui;
mod vm;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let gui_mode = args.iter().any(|arg| arg == "--gui");
    
    // Get the input file name
    let input_file = args.iter()
        .find(|arg| !arg.starts_with("--") && !arg.ends_with(".exe"))
        .and_then(|arg| if arg == &args[0] { None } else { Some(arg) })
        .map(|s| s.as_str())
        .unwrap_or("hiwkhao.asm");

    // For testing, let's create a simple program if no input file exists
    let program = if let Ok(content) = std::fs::read_to_string(input_file) {
        content
    } else {
        // Test program
        vec![
            "LD R0 #42",         // Load immediate integer
            "LD R1 #3.14",       // Load immediate float
            "ST @value R0",      // Store to memory location
            "LD R2 @value",      // Load from memory location
            "ADD.i R3 R0 R2",    // Integer addition
            "ADD.f R4 R1 R1",    // Float addition
            "ST @print R3",      // Print result
            "ST @print R4",      // Print result
        ].join("\n")
    };

    let parsed_program = parse_iloc(&program)
        .map_err(|e| format!("Failed to parse program: {}", e))?;
    
    let mut vm = VM::new(1024);
    vm.load_program(parsed_program);

    if gui_mode {
        // Run in GUI mode
        let vm = Arc::new(Mutex::new(vm));
        gui::run_gui(vm)?;
    } else {
        // Run in normal mode
        vm.run();
    }
    
    Ok(())
}
