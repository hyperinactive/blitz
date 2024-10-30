// TODO: DON'T FORGET TO REMOVE ME
#![allow(dead_code)]

use backend::server::Server;
use cli::processor::Processor;
use dotenv::dotenv;

mod backend;
mod cli;
mod core;
mod op_creator;
mod util;

fn main() {
    dotenv().ok();
    Processor::start();
    // let listener_address = match std::env::var("LISTENER_ADDRESS") {
    //     Ok(env_address) => env_address.to_string(),
    //     Err(_) => "127.0.0.1:8080".to_string(),
    // };

    // Server::new("127.0.0.1:8080");
}
