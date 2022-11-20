use crate::{output, outputln};
use std::io::Write;

pub struct CliProcessor {}

impl CliProcessor {
    pub fn start() {
        let mut input = String::new();

        outputln!("Blitzcrank starting up");

        loop {
            output!("");
            std::io::stdout().flush().unwrap();

            std::io::stdin()
                .read_line(&mut input)
                .expect("Expected valid input");

            let args: Vec<&str> = input.split_whitespace().collect();

            let arg = match args.get(0) {
                Some(arg_0) => arg_0,
                _ => {
                    outputln!("No arguments provided");
                    continue;
                }
            };

            if (*arg) == "exit" {
                break;
            }

            input = "".to_string();
        }
    }
}
