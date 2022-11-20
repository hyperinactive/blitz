use std::collections::{BTreeSet, HashSet};

use super::{index::Index, table::Table};

#[derive(Debug)]
pub struct Database {
    name: String,
    tables: HashSet<Table>,
    index_tree: BTreeSet<Index>,
}

impl Database {
    pub fn new(name: &str, tables: Option<HashSet<Table>>) -> Database {
        Database {
            name: name.to_string(),
            tables: match tables {
                Some(tabs) => tabs,
                None => HashSet::new() as HashSet<Table>,
            },
            index_tree: BTreeSet::new(),
        }
    }
}
