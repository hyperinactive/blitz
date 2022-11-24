// TODO: DON'T FORGET TO REMOVE ME
#![allow(dead_code)]

use crate::core::database::Database;
use backend::server::Server;

mod backend;
mod cli;
mod core;
mod op_creator;
mod util;

struct InMemDb {
    dbs: Vec<Database>,
}

impl InMemDb {}

fn main() {
    // Processor::start();
    Server::new("127.0.0.1:8080");
}
