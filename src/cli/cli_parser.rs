use super::token::Token;

pub struct CliParser {
    token_stream: Vec<Token>,
    index: u128,
}

impl CliParser {
    pub fn new(token_stream: Vec<Token>) -> Self {
        CliParser {
            token_stream,
            index: 0,
        }
    }

    pub fn parse() {}
}
