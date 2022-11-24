use std::collections::{BTreeSet, HashMap};

use super::{index::Index, table::Table};

pub trait DbStruct {
    fn create(name: &str) -> Self
    where
        Self: Sized;
    fn update(&mut self);
    fn delete(&self);
    fn read(&self);
}

#[derive(Debug)]
pub struct Database {
    name: String,
    tables: HashMap<String, Table>,
    index_tree: BTreeSet<Index>,
}

impl Database {}

impl DbStruct for Database {
    fn create(name: &str) -> Self {
        Database {
            name: name.to_string(),
            tables: HashMap::new(),
            index_tree: BTreeSet::new(),
        }
    }

    fn update(&mut self) {
        todo!()
    }

    fn delete(&self) {
        todo!()
    }

    fn read(&self) {
        todo!()
    }
}
