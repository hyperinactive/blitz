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

use std::collections::HashMap;

use lazy_static::lazy_static;

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub t_type: String,
    pub value: Option<String>,
}

impl Token {
    pub fn new(token_type: &str, value: Option<String>) -> Self {
        Token {
            t_type: token_type.to_string(),
            value,
        }
    }
}

lazy_static! {
    pub static ref KEYWORDS: HashMap<&'static str, u8> = {
        let map: HashMap<&'static str, u8> = [
            ("create", 0),
            ("read", 1),
            ("update", 2),
            ("delete", 3),
            ("in", 4),
            ("database", 5),
            ("table", 6),
            ("column", 7),
        ]
        .iter()
        .cloned()
        .collect();
        map
    };
    pub static ref CRUD_KEYWORDS: HashMap<&'static str, u8> = {
        let map: HashMap<&'static str, u8> =
            [("create", 0), ("read", 1), ("update", 2), ("delete", 3)]
                .iter()
                .cloned()
                .collect();
        map
    };
    pub static ref STRUCT_KEYWORDS: HashMap<&'static str, u8> = {
        let map: HashMap<&'static str, u8> = [("database", 0), ("table", 1), ("column", 2)]
            .iter()
            .cloned()
            .collect();
        map
    };
    pub static ref TOKEN_TYPES: HashMap<&'static str, u8> = {
        let map: HashMap<&'static str, u8> = [
            ("EOF", 0),
            ("IDENT", 1),
            ("LBRACE", 2),
            ("RBRACE", 3),
            ("KEYWORD", 4),
        ]
        .iter()
        .cloned()
        .collect();
        map
    };
}
