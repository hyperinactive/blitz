use crate::{output, outputln};
use std::io::Write;

pub struct CliProcessor {}

impl CliProcessor {
    pub fn start() {
        let mut input = String::new();

        outputln!("Fired up and ready to serve");

        loop {
            output!("");
            std::io::stdout().flush().unwrap();

            std::io::stdin()
                .read_line(&mut input)
                .expect("Expected valid input");

            let args: Vec<&str> = input.split_whitespace().collect();

            if args.len() == 0 {
                continue;
            }

            if args[0] == "exit" {
                break;
            }

            input = "".to_string();
        }
    }
}
