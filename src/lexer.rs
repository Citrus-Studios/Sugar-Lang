use crate::tokens::{Tokens, TokensStruct};

pub struct Lexer {
    input: String,
    line: u128,
    char_pos: u128,
    tokens: Vec<TokensStruct>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input,
            line: 1,
            char_pos: 1,
            tokens: vec![],
        }
    }
    /// Runs the first pass of the Lexer
    pub fn first_pass(&mut self) {
        for x in self.input.chars() {
            let tok: Tokens = x.to_string().into();
            match tok {
                Tokens::NewLine => {
                    self.line += 1;
                    self.char_pos = 1;
                }
                _ => {
                    let tok_struct = TokensStruct {
                        token: tok,
                        string: x.to_string(),
                        line: self.line,
                        char_pos: self.char_pos,
                    };
                    self.tokens.push(tok_struct);
                    self.char_pos += 1;
                }
            }
        }
    }
    /// Runs a second pass of the Lexer
    pub fn second_pass(&mut self) {
        let tokens_clone = self.tokens.clone();
        for x in tokens_clone {}
    }
    /// Runs the Lexer
    pub fn run(&mut self) -> Vec<TokensStruct> {
        self.first_pass();
        todo!();
    }
}
