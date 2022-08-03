use crate::{
    ast::{ASTStruct, Symbol, AST},
    tokens::{Tokens, TokensStruct},
};

use tracing::info;

pub struct Parser {
    tokens: Vec<TokensStruct>,
    syntax_tree: Vec<ASTStruct>,
}

impl Parser {
    pub fn new(tokens: Vec<TokensStruct>) -> Self {
        let mut tokens_clone = tokens.clone();
        tokens_clone.push(TokensStruct {
            token: Tokens::Null,
            string: "".to_string(),
            scope: 0,
            line: 0,
            char_pos: 0,
        });
        let tokens = tokens_clone;
        Self {
            tokens,
            syntax_tree: vec![],
        }
    }
    pub fn run(mut self) -> Vec<ASTStruct> {
        let mut last_token: Option<TokensStruct> = None;
        let mut current_scope = 0;
        let mut equality_matched = false;
        for x in self.tokens.clone() {
            let mut matched = false;
            // info!("CurTokens: {:#?} {:#?}", x, last_token);
            match last_token {
                Some(v) => {
                    let y = v.token.clone();
                    let z = x.token.clone();
                    info!("{y:#?}, {z:#?} | {}, {}", v.string, x.string);

                    if y == Tokens::Delimiter {
                        match v.string.as_str() {
                            "{" | "[" | "(" => current_scope += 1,
                            "}" | "]" | ")" => current_scope -= 1,
                            _ => {}
                        }
                    }

                    // ->
                    if y == Tokens::Subtract && z == Tokens::Greater {
                        info!("Arrow ran!");
                        self.syntax_tree.push(ASTStruct {
                            ast: AST::Symbol(Symbol::Arrow),
                            line: x.line,
                            char_pos: x.char_pos,
                            scope: current_scope,
                        });
                        matched = true;
                    }
                    // ==
                    if y == Tokens::Equal && z == Tokens::Equal {
                        info!("Equality ran!");
                        self.syntax_tree.push(ASTStruct {
                            ast: AST::Symbol(Symbol::Equality),
                            line: x.line,
                            char_pos: x.char_pos,
                            scope: current_scope,
                        });
                        matched = true;
                        equality_matched = true;
                    }
                    // =
                    if y == Tokens::Equal && z != Tokens::Equal && !matched {
                        if !equality_matched {
                            info!("Equal ran!");
                            self.syntax_tree.push(ASTStruct {
                                ast: AST::Symbol(Symbol::Equal),
                                line: x.line,
                                char_pos: x.char_pos,
                                scope: current_scope,
                            });
                            matched = true;
                        } else {
                            equality_matched = false;
                        }
                    }
                    // !
                    if y == Tokens::Bang && z != Tokens::Equal {
                        info!("Bang ran!");
                        self.syntax_tree.push(ASTStruct {
                            ast: AST::Symbol(Symbol::Bang),
                            line: x.line,
                            char_pos: x.char_pos,
                            scope: current_scope,
                        });
                        matched = true;
                    }
                    // !=
                    if y == Tokens::Bang && z == Tokens::Equal {
                        info!("InEquality ran!");
                        self.syntax_tree.push(ASTStruct {
                            ast: AST::Symbol(Symbol::InEquality),
                            line: x.line,
                            char_pos: x.char_pos,
                            scope: current_scope,
                        });
                        matched = true;
                    }
                    // dec
                    if y == Tokens::Ident && v.string.as_str() == "dec" {
                        info!("Declare ran!");
                        self.syntax_tree.push(ASTStruct {
                            ast: AST::Declare,
                            line: v.line,
                            char_pos: v.char_pos,
                            scope: current_scope,
                        });
                        matched = true;
                    }
                    // def
                    if y == Tokens::Ident && v.string.as_str() == "def" {
                        info!("Declare ran!");
                        self.syntax_tree.push(ASTStruct {
                            ast: AST::Define,
                            line: v.line,
                            char_pos: v.char_pos,
                            scope: current_scope,
                        });
                        matched = true;
                    }
                    // byte
                    if y == Tokens::Ident && v.string.as_str() == "byte" {
                        info!("Matched Byte");
                        self.syntax_tree.push(ASTStruct {
                            ast: AST::Type("byte".to_string()),
                            line: v.line,
                            char_pos: v.char_pos,
                            scope: current_scope,
                        });
                        matched = true;
                    }
                    // ret
                    if y == Tokens::Ident && v.string.as_str() == "ret" {
                        info!("Return Matched");
                        self.syntax_tree.push(ASTStruct {
                            ast: AST::Return,
                            line: v.line,
                            char_pos: v.char_pos,
                            scope: current_scope,
                        });
                        matched = true;
                    }
                    if y == Tokens::SemiColon {
                        info!("SemiColon Matched");
                        self.syntax_tree.push(ASTStruct {
                            ast: AST::Symbol(Symbol::SemiColon),
                            line: v.line,
                            char_pos: v.char_pos,
                            scope: current_scope,
                        });
                        matched = true;
                    }

                    // Number
                    if y == Tokens::Ident
                        && v.string
                            .chars()
                            .any(|x| x.to_string().parse::<u8>().is_ok())
                    {
                        info!("Number Matched");
                        self.syntax_tree.push(ASTStruct {
                            ast: AST::Byte(v.string.parse::<u8>().expect("That is not a byte")),
                            line: v.line,
                            char_pos: v.char_pos,
                            scope: current_scope,
                        });
                        matched = true;
                    }

                    // Misc Names
                    if !matched && y == Tokens::Ident {
                        info!("Name Matched");
                        self.syntax_tree.push(ASTStruct {
                            ast: AST::Name(v.string),
                            line: v.line,
                            char_pos: v.char_pos,
                            scope: current_scope,
                        });
                    }
                }
                None => {}
            }
            last_token = Some(x.clone());
        }
        return self.syntax_tree.clone();
    }
}
