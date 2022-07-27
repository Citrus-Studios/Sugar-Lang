use crate::{
    ast::{ASTStruct, Symbol, AST},
    tokens::{Tokens, TokensStruct},
};

use tracing::info;

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
                    scope: 1,
                    contents: vec![],
                },
                char_pos: 0,
                line: 0,
            },
        }
    }
    pub fn run(mut self) -> ASTStruct {
        let mut last_token: Option<TokensStruct> = None;
        let mut block = ASTStruct::get_block(&mut self.syntax_tree).unwrap();
        for x in self.tokens.clone() {
            let mut matched = false;
            // info!("CurTokens: {:#?} {:#?}", x, last_token);
            match last_token {
                Some(v) => {
                    let y = v.token.clone();
                    let z = x.token.clone();
                    info!("{y:#?}, {z:#?} | {}, {}", v.string, x.string);

                    // new block
                    if y == Tokens::Delimiter && v.string.as_str() == "{" {
                        block.1.push(ASTStruct {
                            ast: AST::Block {
                                scope: *block.0 + 1,
                                contents: vec![],
                            },
                            line: v.line,
                            char_pos: v.char_pos,
                        });
                        let len = block.1.clone().len();
                        block = ASTStruct::get_block(&mut block.1[len - 1]).unwrap();
                    }
                    // go up a block
                    if y == Tokens::Delimiter && v.string.as_str() == "}" {
                        info!("{}, {}", block.0, block.0.clone() - 1);
                        block = ASTStruct::get_block(&mut self.syntax_tree).unwrap();
                        for i in 0..(*block.0 - 1) {
                            info!("Iteration {i}");
                            let len = block.1.clone().len();
                            block = ASTStruct::get_block(&mut block.1[len - 1]).unwrap();
                        }
                    }

                    // ->
                    if y == Tokens::Subtract && z == Tokens::Greater {
                        info!("Arrow ran!");
                        block.1.push(ASTStruct {
                            ast: AST::Symbol(Symbol::Arrow),
                            line: x.line,
                            char_pos: x.char_pos,
                        });
                        matched = true;
                    }
                    // ==
                    if y == Tokens::Equal && z == Tokens::Equal {
                        info!("Equality ran!");
                        block.1.push(ASTStruct {
                            ast: AST::Symbol(Symbol::Equality),
                            line: x.line,
                            char_pos: x.char_pos,
                        });
                        matched = true;
                    }
                    // =
                    if y == Tokens::Equal && z != Tokens::Equal && !matched {
                        info!("Equal ran!");
                        block.1.push(ASTStruct {
                            ast: AST::Symbol(Symbol::Equal),
                            line: x.line,
                            char_pos: x.char_pos,
                        });
                        matched = true;
                    }
                    // !
                    if y == Tokens::Bang && z != Tokens::Equal {
                        info!("Bang ran!");
                        block.1.push(ASTStruct {
                            ast: AST::Symbol(Symbol::Bang),
                            line: x.line,
                            char_pos: x.char_pos,
                        });
                        matched = true;
                    }
                    // !=
                    if y == Tokens::Bang && z == Tokens::Equal {
                        info!("InEquality ran!");
                        block.1.push(ASTStruct {
                            ast: AST::Symbol(Symbol::InEquality),
                            line: x.line,
                            char_pos: x.char_pos,
                        });
                        matched = true;
                    }
                    // dec
                    if y == Tokens::Ident && v.string.as_str() == "dec" {
                        info!("Declare ran!");
                        block.1.push(ASTStruct {
                            ast: AST::Declare,
                            line: v.line,
                            char_pos: v.char_pos,
                        });
                        matched = true;
                    }
                    // def
                    if y == Tokens::Ident && v.string.as_str() == "def" {
                        info!("Declare ran!");
                        block.1.push(ASTStruct {
                            ast: AST::Define,
                            line: v.line,
                            char_pos: v.char_pos,
                        });
                        matched = true;
                    }
                    // byte
                    if y == Tokens::Ident && v.string.as_str() == "byte" {
                        info!("Matched Byte");
                        block.1.push(ASTStruct {
                            ast: AST::Type("byte".to_string()),
                            line: v.line,
                            char_pos: v.char_pos,
                        });
                        matched = true;
                    }
                    // ret
                    if y == Tokens::Ident && v.string.as_str() == "ret" {
                        info!("Return Matched");
                        block.1.push(ASTStruct {
                            ast: AST::Return,
                            line: v.line,
                            char_pos: v.char_pos,
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
                        block.1.push(ASTStruct {
                            ast: AST::Byte(v.string.parse::<u8>().expect("That is not a byte")),
                            line: v.line,
                            char_pos: v.char_pos,
                        });
                        matched = true;
                    }

                    // Misc Names
                    if !matched && y == Tokens::Ident {
                        info!("Name Matched");
                        block.1.push(ASTStruct {
                            ast: AST::Name(v.string),
                            line: v.line,
                            char_pos: v.char_pos,
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
