use super::database::Database;
use std::collections::{HashSet, LinkedList};

// needs to have a list of databases
//
pub struct Blitzcrank {
    // no duplicates, databases unique
    dbs: HashSet<Database>,      // TODO: something like hash map maybe better
    actions: LinkedList<String>, // TODO: action queue -> transactions
}

// TODO: needs to open a file and read from it
// needs to read databases and init the database
impl Blitzcrank {
    // TODO:
    pub fn init() {
        todo!("call file system and set databases, create action queue");
    }

    pub fn shutdown() {
        todo!("ensure everything's written down before exiting");
    }
}
