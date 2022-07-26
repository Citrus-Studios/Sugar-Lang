use crate::tokens::TokensStruct;

pub struct Parser {
    tokens: Vec<TokensStruct>,
}

impl Parser {
    pub fn new(tokens: Vec<TokensStruct>) -> Self {
        Self { tokens }
    }
    pub fn run(&mut self) {}
}
