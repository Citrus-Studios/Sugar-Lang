use crate::tokens::{Tokens, TokensStruct};

pub struct Lexer {
    input: String,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self { input }
    }
    pub fn run(&mut self) -> Vec<TokensStruct> {
        for x in self.input.chars() {
            let tok: Tokens = x.to_string().into();
        }
        todo!();
    }
}
