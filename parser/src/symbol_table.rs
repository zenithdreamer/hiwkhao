use csv::Writer;
use scanner::grammar::Token;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

use crate::{Expr, ParseError};

#[derive(Debug, Clone)]
pub enum VariableType {
    INT,
    REAL,
    LIST(Box<VariableType>),
}

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
    //tokens: Vec<Token>,
    //pos: usize,
    variables: HashMap<String, VariableType>,
    //current_line: usize,
    //current_column: usize,
    //token_positions: Vec<usize>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            entries: Vec::new(),
            //tokens: Vec::new(),
            //pos: 0,
            variables: HashMap::new(),
            //current_line: 1,
            //current_column: 1,
            //token_positions: Vec::new(),
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

    pub fn process_parsed_expressions(&mut self, parsed_results: Vec<Result<Expr, ParseError>>) {
        let mut current_pos = 0;

        for (line_number, result) in parsed_results.iter().enumerate() {
            if let Ok(expr) = result {
                match expr {
                    Expr::Assignment(var_name, value_expr) => {
                        let (value_type, value, length) = match &**value_expr {
                            Expr::Int(n) => {
                                self.variables.insert(var_name.clone(), VariableType::INT);
                                (
                                    Token::INT(n.to_string()),
                                    n.to_string(),
                                    n.to_string().len(),
                                )
                            }
                            Expr::Float(n) => {
                                self.variables.insert(var_name.clone(), VariableType::REAL);
                                (
                                    Token::REAL(n.to_string()),
                                    n.to_string(),
                                    n.to_string().len(),
                                )
                            }
                            Expr::List(elements) => {
                                // Determine list element type dynamically (e.g., INT or REAL) by inspecting the elements
                                // Even all element are f64 but if non of them has a decimal point, we can assume it's an INT
                                let element_type = if elements.iter().all(|e| e.fract() == 0.0) {
                                    VariableType::INT
                                } else {
                                    VariableType::REAL
                                };
                                self.variables.insert(
                                    var_name.clone(),
                                    VariableType::LIST(Box::new(element_type)),
                                );

                                (Token::LIST, "Array".to_string(), "Array".len())
                            }

                            Expr::ListAccess(list_name, index) => {
                                let list_type = self.variables.get(list_name);
                                let element_type = match list_type {
                                    Some(VariableType::LIST(inner_type)) => inner_type.as_ref(),
                                    _ => &VariableType::INT, // Default to INT if unknown
                                };

                                let index_value = if let Expr::Int(n) = **index {
                                    n.to_string()
                                } else {
                                    "unknown".to_string()
                                };

                                let token_type = match element_type {
                                    VariableType::INT => Token::INT(index_value.clone()),
                                    VariableType::REAL => Token::REAL(index_value.clone()),
                                    _ => Token::LIST,
                                };

                                (
                                    token_type,
                                    format!("{}[{}]", list_name, index_value),
                                    list_name.len() + index_value.len() + 2,
                                )
                            }
                            _ => continue,
                        };

                        self.insert(
                            var_name.clone(),
                            line_number + 1,
                            current_pos,
                            length,
                            value_type,
                            value,
                        );

                        current_pos += length + 1;
                    }
                    _ => (),
                }
            }
        }
    }

    pub fn output(&self) -> Vec<String> {
        let mut symbol_table_output = Vec::new();

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
        let file = File::create(filename)?;
        let mut wtr = Writer::from_writer(file);

        wtr.write_record(&[
            "Lexeme",
            "Line Number",
            "Start Position",
            "Length",
            "Type",
            "Value",
        ])?;

        for entry in &self.entries {
            let type_name = format!("{:?}", entry.value_type);
            let trimmed_type = type_name.split('(').next().unwrap_or(&type_name);

            wtr.write_record(&[
                &entry.lexeme,
                &entry.line_number.to_string(),
                &entry.start_pos.to_string(),
                &entry.length.to_string(),
                trimmed_type,
                &entry.value,
            ])?;
        }

        wtr.flush()?;
        Ok(())
    }
}
