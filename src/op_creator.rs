use std::{
    borrow::{Borrow, BorrowMut},
    cell::{Cell, RefCell},
    rc::Rc,
    sync::{Arc, Mutex},
};

use crate::{
    cli::token::{Token, CRUD_KEYWORDS, STRUCT_KEYWORDS},
    core::database::{Database, DbStruct},
};

pub enum StatementType {
    Read,
    Create,
    Update,
    Delete,
    In,
}

pub enum Structure {
    Database,
    Table,
    Column,
}

pub struct Statement {
    stat_type: StatementType,
    struct_type: Structure,
    ident: String,
}

struct StatementConsumer {}

impl StatementConsumer {
    pub fn execute(statement: Statement) {
        let shared_ident = Rc::new(RefCell::new(StatementConsumer::get_ident(
            statement.struct_type,
            statement.ident,
        )));

        match statement.stat_type {
            StatementType::Create => StatementConsumer::execute_create(shared_ident),
            StatementType::Read => StatementConsumer::execute_create(shared_ident),
            StatementType::Update => StatementConsumer::execute_create(shared_ident),
            StatementType::Delete => StatementConsumer::execute_create(shared_ident),
            StatementType::In => (),
        }
    }

    // TODO: watch out, don't want you stepping on this
    // NOTE: maybe too early to worry about concurrency?
    fn get_ident(s: Structure, ident: String) -> impl DbStruct {
        // TODO: dude, implement this properly
        Database::create("hi, lol")
    }

    fn execute_create(s: Rc<RefCell<impl DbStruct>>) {
        // let ptr_clone = s.clone();
        // ptr_clone.lock().unwrap();
        todo!();
    }

    fn execute_read(s: Rc<RefCell<impl DbStruct>>) {
        // let ptr_clone = s.clone();
        // ptr_clone.lock().unwrap();
        todo!();
    }

    fn execute_update(s: Rc<RefCell<impl DbStruct>>) {
        // let ptr_clone = s.clone();
        // ptr_clone.lock().unwrap();
        todo!();
    }

    fn execute_delete(s: Rc<RefCell<impl DbStruct>>) {
        // let ptr_clone = s.clone();
        // ptr_clone.lock().unwrap();
        todo!();
    }
}

pub struct OpCreator {}

impl OpCreator {
    pub fn input(tokens: Vec<Token>) {
        let mut i: usize = 0;
        let tokens_len = tokens.len();

        let mut current_struct: Rc<Token>;
        let mut current_action: Rc<Token>;

        loop {
            if i == tokens_len {
                break;
            }

            if tokens[i].t_type == "KEYWORD" {
                match tokens[i].value.clone() {
                    Some(v) => {
                        if CRUD_KEYWORDS.contains(&*v) {
                            current_action = Rc::new(tokens[i].clone());
                        }

                        if STRUCT_KEYWORDS.contains(&*v) {
                            current_struct = Rc::new(tokens[i].clone());
                        }
                    }
                    None => (),
                }
            }

            i += 1;
        }
    }
}
