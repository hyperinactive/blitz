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

use std::collections::HashSet;

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
    pub static ref KEYWORDS: HashSet<&'static str> = {
        let map: HashSet<&'static str> = [
            ("create"),
            ("read"),
            ("update"),
            ("delete"),
            ("in"),
            ("database"),
            ("table"),
            ("column"),
        ]
        .iter()
        .cloned()
        .collect();
        map
    };
    pub static ref CRUD_KEYWORDS: HashSet<&'static str> = {
        let map: HashSet<&'static str> = [("create"), ("read"), ("update"), ("delete")]
            .iter()
            .cloned()
            .collect();
        map
    };
    pub static ref STRUCT_KEYWORDS: HashSet<&'static str> = {
        let map: HashSet<&'static str> = [("database"), ("table"), ("column")]
            .iter()
            .cloned()
            .collect();
        map
    };
    pub static ref TOKEN_TYPES: HashSet<&'static str> = {
        let map: HashSet<&'static str> = [("EOF"), ("IDENT"), ("LBRACE"), ("RBRACE"), ("KEYWORD")]
            .iter()
            .cloned()
            .collect();
        map
    };
}
