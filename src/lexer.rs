use crate::tokens::{Tokens, TokensStruct};

pub struct Lexer {
    input: String,
    line: usize,
    char_pos: usize,
    tokens: Vec<TokensStruct>,
    scope: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input,
            line: 1,
            char_pos: 1,
            tokens: vec![],
            scope: 0,
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
                        scope: 0,
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
        let mut last_x = None;
        for (_i, x) in tokens_clone.iter().enumerate() {
            match x.token {
                Tokens::Ident => {
                    ident_mode = true;
                    ident_cache.push_str(x.string.as_str());
                    last_x = Some(x.clone());
                }
                _ => {
                    if ident_mode {
                        let last_x_unwrapped = last_x.clone().unwrap();
                        self.tokens.push(TokensStruct {
                            token: last_x_unwrapped.clone().token,
                            string: ident_cache,
                            char_pos: last_x_unwrapped.char_pos,
                            line: last_x_unwrapped.line,
                            scope: self.scope,
                        });
                        ident_cache = String::new();
                        ident_mode = false;
                    }
                    if x.token != Tokens::Space {
                        self.tokens.push(TokensStruct {
                            token: x.token.clone(),
                            string: x.string.clone(),
                            line: x.line,
                            char_pos: x.char_pos,
                            scope: self.scope,
                        });
                    }
                    if x.token == Tokens::Delimiter {
                        match x.string.as_str() {
                            "{" | "[" | "(" => self.scope += 1,
                            "}" | "]" | ")" => self.scope -= 1,
                            _ => {}
                        }
                    }
                    last_x = Some(x.clone());
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
