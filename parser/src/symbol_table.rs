use scanner_lib::grammar::Token;
use logos::Logos;
use std::error::Error;
use std::fs::File;
use csv::Writer;

#[derive(Debug)]
pub struct SymbolTableEntry {
    lexeme: String,
    line_number: usize,
    start_pos: usize,
    length: usize,
    value_type: Token,
    value: String,
}

#[derive(Debug)]
pub struct SymbolTable {
    entries: Vec<SymbolTableEntry>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            entries: Vec::new()
        }
    }

    pub fn insert(
        &mut self,
        lexeme: String,
        line_number: usize,
        start_pos: usize,
        length: usize,
        value_type: Token,
        value: String,
    ) {
        let entry = SymbolTableEntry {
            lexeme,
            line_number,
            start_pos,
            length,
            value_type,
            value,
        };
        self.entries.push(entry);
    }

    pub fn output(&self) -> Vec<String> {
        let mut symbol_table_output: Vec<String> = vec![];

        for entry in &self.entries {
            symbol_table_output.push(format!(
                "{},{},{},{},{:?},{}",
                entry.lexeme,
                entry.line_number,
                entry.start_pos,
                entry.length,
                entry.value_type,
                entry.value
            ));
        }

        symbol_table_output
    }

    pub fn write_to_csv(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        // Open the file for writing
        let file = File::create(filename)?;

        // Create a CSV writer
        let mut wtr = Writer::from_writer(file);

        // Write the header
        wtr.write_record(&["Lexeme", "Line Number", "Start Position", "Length", "Type", "Value"])?;

        // Write the data for each entry in the symbol table
        for entry in &self.entries {
            wtr.write_record(&[
                &entry.lexeme,
                &entry.line_number.to_string(),
                &entry.start_pos.to_string(),
                &entry.length.to_string(),
                &format!("{:?}", entry.value_type),
                &entry.value,
            ])?;
        }

        // Flush and close the writer
        wtr.flush()?;

        Ok(())
    }
}

pub fn get_symbol_table(input: &str) -> SymbolTable {
    let mut symbol_table = SymbolTable::new();

    for (line_number, line) in input.lines().enumerate() {
        let tokens = tokenize(line);
        let line_number = line_number + 1;

        for (i, (_slice, token)) in tokens.iter().enumerate() {
            if *token == Token::ASSIGN {
                if i > 0 {
                    let (left_token_slice, _left_token) = &tokens[i - 1];
                    let left_start_pos = line.find(left_token_slice).unwrap_or(0);
                    let left_length = left_token_slice.len();

                    if i + 1 < tokens.len() {
                        let (right_token_slice, right_token) = &tokens[i + 1];
                        if matches!(right_token, Token::INT | Token::REAL | Token::LIST) {
                            let mut value = right_token_slice.to_string();
                            if value.starts_with("list") {
                                value = "Array".to_string();
                            }
    
                            symbol_table.insert(
                                left_token_slice.to_string(),  
                                line_number,                  
                                left_start_pos + 1,            
                                left_length,                   
                                right_token.clone(),           
                                value,                       
                            );
                        }
                    }
                }
            }
        }
    }
    symbol_table.write_to_csv("hiwkhao.csv");

    symbol_table
}

pub fn tokenize(input: &str) -> Vec<(String, Token)> {
    let mut lexer = Token::lexer(input);
    let mut tokens = Vec::new();

    while let Some(token) = lexer.next() {
        let slice = lexer.slice().to_string();
        
        match token {
            Ok(tok) => tokens.push((slice, tok)),
            Err(_) => tokens.push((slice, Token::ERR)),
        }
    }

    tokens
}

