use crate::tokens::{CollectCharStrings, SameVecType, Tokens, TokensStruct};

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
        self.tokens.clear();
        let mut ident_mode = false;
        let mut ident_cache = String::new();
        for (_i, x) in tokens_clone.iter().enumerate() {
            match x.token {
                Tokens::Ident => {
                    ident_mode = true;
                    ident_cache.push_str(x.string.as_str());
                }
                _ => {
                    if ident_mode {
                        self.tokens.push(TokensStruct {
                            token: x.clone().token,
                            string: ident_cache,
                            char_pos: x.char_pos,
                            line: x.line,
                        });
                        ident_cache = String::new();
                        ident_mode = false;
                    } else if x.token != Tokens::Space {
                        self.tokens.push(x.clone());
                    }
                }
            }
        }
    }
    /// Runs the Lexer
    pub fn run(&mut self) -> Vec<TokensStruct> {
        self.first_pass();
        self.second_pass();
        self.tokens.clone()
    }
}
