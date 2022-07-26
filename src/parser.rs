use crate::{
    ast::{ASTStruct, AST},
    tokens::{Tokens, TokensStruct},
};

pub struct Parser {
    tokens: Vec<TokensStruct>,
    syntax_tree: ASTStruct,
}

impl Parser {
    pub fn new(tokens: Vec<TokensStruct>) -> Self {
        Self {
            tokens,
            syntax_tree: ASTStruct {
                ast: AST::Block {
                    scope: 0,
                    contents: vec![],
                },
                char_pos: 0,
                line: 0,
            },
        }
    }
    pub fn run(&mut self) {
        let mut last_token: Option<TokensStruct> = None;
        for x in self.tokens.clone() {
            if match last_token {
                // Check if it's an arrow
                Some(v) => v.token == Tokens::Minus && x.token == Tokens::Greater,
                None => false,
            } {}
            last_token = Some(x.clone());
        }
    }
}
