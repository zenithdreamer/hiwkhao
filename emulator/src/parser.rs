pub fn parse_iloc(input: &str) -> Result<Vec<String>, String> {
    let mut program = Vec::new();
    
    for line in input.lines() {
        let line = line.trim();
        // Skip empty lines and comments
        if line.is_empty() || line.starts_with("//") {
            continue;
        }
        program.push(line.to_string());
    }
    
    Ok(program)
} 