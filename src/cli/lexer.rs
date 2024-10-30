use std::{
    borrow::{Borrow, BorrowMut},
    cell::{Cell, RefCell},
    rc::Rc,
};

use super::token::{Token, CRUD_KEYWORDS, STRUCT_KEYWORDS};

#[derive(PartialEq, Debug)]
pub enum StateType {
    INIT,
    CRUD,
    STRUCT,
    IDENT,
    LBRACE,
    RBRACE,
    EOF,
}

pub struct LexerState {
    state: StateType,
    brace_count: i64,
}

impl LexerState {
    pub fn new() -> Self {
        LexerState {
            state: StateType::INIT,
            brace_count: 0,
        }
    }

    pub fn transition(&mut self, state: StateType, token_index: usize) -> Result<(), String> {
        match state {
            StateType::INIT => {
                panic!("Lexer error. Cannot set to init state.");
            }
            StateType::CRUD => {
                return if self.state == StateType::INIT
                    || self.state == StateType::RBRACE
                    || self.state == StateType::LBRACE
                {
                    self.state = state;
                    Ok(())
                } else {
                    Err(format!("Lexer error. State transition failed. Expected INIT or RBRACE. State machine in: {:?}. Token index: {}", self.state, token_index))
                };
            }
            StateType::STRUCT => {
                return if self.state == StateType::CRUD {
                    self.state = state;
                    Ok(())
                } else {
                    Err(format!("Lexer error. State transition failed. Expected CRUD. State machine in: {:?}. Token index: {}", self.state, token_index))
                };
            }
            StateType::IDENT => {
                return if self.state == StateType::STRUCT {
                    self.state = state;
                    Ok(())
                } else {
                    Err(format!("Lexer error. State transition failed. Expected STRUCT. State machine in: {:?}. Token index: {}", self.state, token_index))
                };
            }
            StateType::LBRACE => {
                self.brace_count += 1;
                return if self.state == StateType::IDENT {
                    self.state = state;
                    Ok(())
                } else {
                    Err(format!("Lexer error. State transition failed. Expected IDENT. State machine in: {:?}. Token index: {}", self.state, token_index))
                };
            }

            // TODO: needs to be expanded
            // for now only allows empty braces {}
            StateType::RBRACE => {
                self.brace_count -= 1;
                return if self.state == StateType::LBRACE || self.state == StateType::RBRACE {
                    self.state = state;
                    Ok(())
                } else {
                    Err(format!("Lexer error. State transition failed. Expected BLOCK. State machine in: {:?}. Token index: {}", self.state, token_index))
                };
            }
            StateType::EOF => {
                if self.brace_count != 0 {
                    return Err("Lexer error. Invalid brace count.".to_string());
                }
                return Ok(());
            }
        };
    }
}

/// # Lexer
/// NOTE: does both input parsing and tokenization.
pub struct Lexer {}

impl Lexer {
    pub fn is_string_numeric(str: String) -> bool {
        for c in str.chars() {
            if !c.is_numeric() {
                return false;
            }
        }
        return true;
    }
    // keyword keyword ident {
    //     keyword keyword ident {
    //         keyword keyword ident {
    //             ....
    //         }
    //         keyword keyword ident {
    //             ....
    //         }
    //         ...
    //      }
    // }

    // create database ident {
    //     create table ident {
    //         create column ident {
    //             { type STRING }
    //         }
    //         create column ident {
    //             ....
    //         }
    //         ...
    //      }
    // }

    pub fn parse(input: Vec<&str>) -> Result<Vec<Token>, String> {
        let mut mapped_input = Vec::new();
        for i in input {
            mapped_input.push(String::from(i))
        }
        mapped_input.push("EOF".to_string());

        let mut j: usize = 0;
        let mut state_machine = LexerState::new();
        let mut token_output: Vec<Token> = Vec::new();

        loop {
            let current = mapped_input[j].to_owned();

            if current == "EOF" {
                state_machine.transition(StateType::EOF, j)?;
                token_output.push(Token::new("EOF", None));
                break;
            }

            if CRUD_KEYWORDS.contains(String::as_str(&current)) {
                state_machine.transition(StateType::CRUD, j)?;
                token_output.push(Token::new("KEYWORD", Some(current)));
                j += 1;
                continue;
            }

            if STRUCT_KEYWORDS.contains(String::as_str(&current)) {
                state_machine.transition(StateType::STRUCT, j)?;
                token_output.push(Token::new("KEYWORD", Some(current)));
                j += 1;
                continue;
            }

            if current == "{" {
                state_machine.transition(StateType::LBRACE, j)?;
                token_output.push(Token::new("LBRACE", None));
                j += 1;
                continue;
            }

            if current == "}" {
                state_machine.transition(StateType::RBRACE, j)?;
                token_output.push(Token::new("RBRACE", None));
                j += 1;
                continue;
            }

            state_machine.transition(StateType::IDENT, j)?;
            token_output.push(Token::new("IDENT", Some(current)));
            j += 1;
        }

        Ok(token_output)
    }

    pub fn parse_raw_string(input: String) -> Result<Vec<Token>, String> {
        let mut token_output: Vec<Token> = Vec::new();

        let i_len = input.len();
        let mut i: usize = 0;

        let mut temp_input = String::from("");
        let mut input_iter = input.chars().enumerate();

        // for i in input_iter.len {
        //     println!("{}", i);
        // }

        let mut index: usize = 0;
        let input_length: usize = input.bytes().len();

        // for c in input_iter {
        //     let peek = input_iter.peek();

        //     if Self::is_string_numeric(temp_input.to_string()) {
        //         match peek {
        //             Some(v) => {
        //                 if Self::is_string_numeric(v.to_string()) {
        //                     token_output.push(Token::new("NUM", Some(v.to_string())));
        //                 }
        //             }
        //             None => continue,
        //         }
        //     }

        //     // if (temp_input.chars() && !isNum(peek)) {
        //     //     this.tokens.push({
        //     //         type: TokenType.NUM,
        //     //         value: tempString.trim(),
        //     //     });
        //     //     tempString = "";
        //     //     continue;
        //     // }
        // }

        Ok(token_output)
    }
}

// ***************************************************************************
// TESTS
// ***************************************************************************

#[cfg(test)]
mod tests {
    use crate::{cli::token::Token, util::vec_compare};

    use super::Lexer;

    #[test]
    fn parse() {
        let input = vec!["create", "database", "jojo"];
        let output = Lexer::parse(input).unwrap();

        let expected = vec![
            Token {
                t_type: String::from("KEYWORD"),
                value: Some(String::from("create")),
            },
            Token {
                t_type: String::from("KEYWORD"),
                value: Some(String::from("database")),
            },
            Token {
                t_type: String::from("IDENT"),
                value: Some(String::from("jojo")),
            },
            Token {
                t_type: String::from("EOF"),
                value: None,
            },
        ];

        assert!(vec_compare(&output, &expected));
    }

    #[test]
    fn parse_complex() {
        let input = vec![
            "create", "database", "jojo", "{", "create", "table", "jotaro", "{", "}", "}",
        ];
        let output = Lexer::parse(input).unwrap();

        let expected = vec![
            Token::new("KEYWORD", Some("create".to_string())),
            Token::new("KEYWORD", Some("database".to_string())),
            Token::new("IDENT", Some("jojo".to_string())),
            Token::new("LBRACE", None),
            Token::new("KEYWORD", Some("create".to_string())),
            Token::new("KEYWORD", Some("table".to_string())),
            Token::new("IDENT", Some("jotaro".to_string())),
            Token::new("LBRACE", None),
            Token::new("RBRACE", None),
            Token::new("RBRACE", None),
            Token::new("EOF", None),
        ];

        assert!(output.eq(&expected));
    }

    #[test]
    #[should_panic(expected = "Lexer error. Invalid brace count.")]
    fn panic_at_braces() {
        let input = vec!["create", "database", "jojo", "{", "}", "}"];
        Lexer::parse(input).unwrap();
    }

    #[test]
    #[should_panic(
        expected = "Lexer error. State transition failed. Expected IDENT. State machine in: INIT. Token index: 0"
    )]
    fn panic_at_init() {
        let input = vec!["{", "create", "database", "jojo", "{", "}", "}"];
        Lexer::parse(input).unwrap();
    }
}
