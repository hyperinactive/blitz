use crate::{cli::lexer::Lexer, output, outputln};
use std::{
    io::{self, Write},
    rc::Rc,
};
use crate::core::database::Database;

pub struct Processor {}

impl Processor {
    fn parse_args(input: &mut String) -> Vec<&str> {
        io::stdin().read_line(input).expect("Expected valid input");

        let args = input.split_whitespace().collect();
        args
    }

    pub fn start() {
        outputln!("Fired up and ready to serve");
        let input = Rc::new(String::new());

        loop {
            let mut input_ptr = Rc::clone(&input);
            output!("");
            io::stdout().flush().unwrap();

            let args: Vec<&str> = Processor::parse_args(Rc::make_mut(&mut input_ptr));

            if args.len() == 0 {
                continue;
            }

            if args[0] == "exec" || args[0] == "run" {
                outputln!("[EXEC MODE]");
                output!("");
                let mut exec_input = String::new();
                io::stdout().flush().unwrap();

                let exec_args = Processor::parse_args(&mut exec_input);
                println!("{:?}", exec_args);
                match Lexer::parse(exec_args) {
                    Ok(tokens) => println!("{:#?}", tokens),
                    Err(e) => outputln!(e),
                };
                continue;
            }

            if args[0] == "exit" {
                break;
            }
        }
    }
}
