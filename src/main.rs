// TODO: DON'T FORGET TO REMOVE ME
#![allow(dead_code)]
use cli::cli_processor::CliProcessor;

mod cli;
mod core;
mod util;

fn main() {
    CliProcessor::start();
}
