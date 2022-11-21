use crate::{cli::lexer::Lexer, output, outputln};
use std::io::Write;

pub struct CliProcessor {}

impl CliProcessor {
    fn parse_args<'a>(input: &'a mut String) -> Vec<&'a str> {
        std::io::stdin()
            .read_line(input)
            .expect("Expected valid input");

        let args = input.split_whitespace().collect();
        args
    }

    pub fn start() {
        outputln!("Fired up and ready to serve");

        loop {
            let mut input = String::new();

            output!("");
            std::io::stdout().flush().unwrap();

            let args: Vec<&str> = CliProcessor::parse_args(&mut input);

            if args.len() == 0 {
                continue;
            }

            if args[0] == "exec" || args[0] == "run" {
                outputln!("[EXEC MODE]");
                output!("");
                let mut exec_input = String::new();
                std::io::stdout().flush().unwrap();

                let exec_args = CliProcessor::parse_args(&mut exec_input);
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
