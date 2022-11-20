// ok so I need to make a language here

// blitz keyword
// for using engine commands unrelated to actual database operations

// blitz help
// blitz show_dbs
// blitz show_users

// create keyword -> make dbs, tables, columns
// update keyword -> update dbs(NOTE: what do I update though), tables, columns

// create db *db_name*

// multi table creation?

// in keyword -> selector

// in db *db_name* {
//     create table *table_name* {...}
//     create table *table_name* {...}
// }

// in db *db_name* create table *table_name* {
//     create column *column_name* {...}
// }

// ...
// create column *column_name* {
//     type: ColumnType
//     unique
//     primary_key
// }
// ...

// in db *db_name* in table *table_name* create column *column_name*

use std::slice::SliceIndex;

use crate::output;

#[derive(Debug, PartialEq)]
pub struct Token {
    t_type: String,
    // most token types won't need a value
    // needed for keywords and operators though
    value: Option<String>, // NOTE: we clear this hurdle when we get to it
}

pub static KEYWORDS: [&str; 11] = [
    "create", "update", "delete", "in", "database", "table", "column", "unique", "p_key", "f_key",
    "default",
];

pub static TOKEN_TYPES: [&str; 5] = ["EOF", "IDENT", "LBRACE", "RBRACE", "KEYWORD"];

/// ### Tokenizer
/// Takes parsed string input vector and returns token output.
pub struct Tokenizer {
    input: Vec<String>,
    index: usize,
}

impl Tokenizer {
    pub fn new(input: Vec<&str>) -> Self {
        let mut mapped_input = Vec::new();

        if (*input.last().unwrap()) != "EOF" {
            panic!("Invalid input. No EOF found");
        }

        for i in input {
            mapped_input.push(String::from(i))
        }

        Tokenizer {
            index: 0,
            input: mapped_input,
        }
    }

    pub fn parse(&mut self) -> Vec<Token> {
        let mut output: Vec<Token> = Vec::new();

        for i in self.input.iter() {
            if TOKEN_TYPES.contains(&i.as_str()) {
                output.push(Token {
                    t_type: i.to_string(),
                    value: None,
                });
                continue;
            }

            if KEYWORDS.contains(&i.as_str()) {
                output.push(Token {
                    t_type: "KEYWORD".to_string(),
                    value: Some(i.to_string()),
                });
                continue;
            };

            output.push(Token {
                t_type: "IDENT".to_string(),
                value: Some(i.to_string()),
            });
        }

        output
    }
}

// ***************************************************************************
// TESTS
// ***************************************************************************

#[cfg(test)]
mod tests {
    use super::{Token, Tokenizer};

    fn vec_compare<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
        let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
        matching == a.len() && matching == b.len()
    }

    #[test]
    fn parse_input() {
        let input = vec!["create", "database", "hello_there", "EOF"];
        let tokens = Tokenizer::new(input).parse();

        let mut expected = vec![
            Token {
                t_type: "KEYWORD".to_string(),
                value: Some("create".to_string()),
            },
            Token {
                t_type: "KEYWORD".to_string(),
                value: Some("database".to_string()),
            },
            Token {
                t_type: "IDENT".to_string(),
                value: Some("hello_there".to_string()),
            },
            Token {
                t_type: "EOF".to_string(),
                value: None,
            },
        ];

        assert!(vec_compare(&tokens, &expected), "These don't match");

        // assert_eq!(tokens, expected);
    }

    #[test]
    #[should_panic(expected = "Invalid input. No EOF found")]
    fn no_eof() {
        let input = vec!["create", "database", "hello_there"];
        Tokenizer::new(input);
        // assert_eq!(tokens, expected);
    }
}
