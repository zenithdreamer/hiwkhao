use csv::Writer;
use scanner_lib::grammar::Token;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

use crate::{Expr, TokenInfo};

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
    tokens: Vec<Token>,
    pos: usize,
    variables: HashMap<String, Expr>,
    current_line: usize,
    current_column: usize,
    token_positions: Vec<usize>,
}

impl SymbolTable {
    pub fn new(tokens: Vec<Token>) -> Self {
        SymbolTable {
            entries: Vec::new(),
            tokens,
            pos: 0,
            variables: HashMap::new(),
            current_line: 1,
            current_column: 1,
            token_positions: Vec::new(),
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
        wtr.write_record(&[
            "Lexeme",
            "Line Number",
            "Start Position",
            "Length",
            "Type",
            "Value",
        ])?;

        // Write the data for each entry in the symbol table
        for entry in &self.entries {
            let type_name = format!("{:?}", entry.value_type);
            let trimmed_type = type_name.split('(').next().unwrap_or(&type_name); // Get only the type name
            wtr.write_record(&[
                &entry.lexeme,
                &entry.line_number.to_string(),
                &entry.start_pos.to_string(),
                &entry.length.to_string(),
                trimmed_type,
                &entry.value,
            ])?;
        }

        // Flush and close the writer
        wtr.flush()?;

        Ok(())
    }

    pub fn get_symbol_table(&mut self, input: logos::Lexer<'_, Token>) -> &mut SymbolTable {
        let tokens: Vec<_> = input.clone().collect();

        // Split tokens into lines and track positions
        let mut current_line = 1;
        let mut lines: Vec<(Vec<Token>, Vec<usize>)> = Vec::new();
        let mut current_line_tokens = Vec::new();
        let mut current_line_positions = Vec::new();
        let mut column = 1;

        // First, organize tokens into lines
        for token in &tokens {
            match &token {
                Ok(Token::NEWLINE) => {
                    if !current_line_tokens.is_empty() {
                        lines.push((current_line_tokens, current_line_positions));
                        current_line_tokens = Vec::new();
                        current_line_positions = Vec::new();
                    }
                    column = 1;
                }
                Ok(Token::WHITESPACE) => {
                    column += 1;
                }
                _ => {
                    if let Ok(tok) = token {
                        current_line_positions.push(column);
                        current_line_tokens.push(tok.clone());
                        column += TokenInfo::token_length(&current_line_tokens.last().unwrap());
                    } else {
                        current_line_positions.push(column);
                        current_line_tokens.push(Token::ERR);
                        column += TokenInfo::token_length(&current_line_tokens.last().unwrap());
                    }
                }
            }
        }

        // Add the last line if it doesn't end with a newline
        if !current_line_tokens.is_empty() {
            lines.push((current_line_tokens, current_line_positions));
        }

        // Process each line's tokens
        for (line_num, (line_tokens, positions)) in lines.iter().enumerate() {
            let line_tokens_vec: Vec<_> = line_tokens.iter().collect();

            // Process assignments within the current line only
            for i in 0..line_tokens_vec.len() {
                if *line_tokens_vec[i] == Token::ASSIGN {
                    if i > 0 {
                        let left_token = line_tokens_vec[i - 1];
                        let left_start_pos = positions[i - 1];
                        let left_length = left_token.to_string().len();

                        if i + 1 < line_tokens_vec.len() {
                            let right_token = line_tokens_vec[i + 1];
                            if matches!(right_token, Token::INT(_) | Token::REAL(_) | Token::LIST) {
                                let mut value = right_token.to_string();
                                if value.starts_with("list") {
                                    value = "Array".to_string();
                                }

                                self.insert(
                                    left_token.to_string(),
                                    line_num + 1,
                                    left_start_pos,
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

        self
    }
}
