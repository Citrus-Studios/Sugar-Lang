use crate::ast::{ASTStruct, Symbol, AST};
use tracing::info;

#[derive(Debug, Clone)]
pub enum RID {
    Byte(u8),
    Arg(String),
    // Declaring a function
    FunctionDeclare(String),
    FunctionDeclareStart,
    FunctionDeclareEnd,
    // Defining a function
    FunctionDefined(String),
    FunctionDefinedArgsStart,
    FunctionDefinedArgsEnd,
    FunctionDefinedContentsStart,
    FunctionDefinedContentsEnd,
    // Var
    Var(String, Box<RID>),
    // Symbol
    Symbol(Symbol),
    // Comparison
    Comparison(Box<RID>, Box<RID>, Box<RID>),
    // If Block
    If(Box<RID>),
    IfStart,
    IfEnd,
    // If Else Block
    IfElse(Box<RID>),
    IfElseStart,
    IfElseEnd,
    // Else Block
    Else,
    ElseStart,
    ElseEnd,
    // Return
    Return(Box<RID>),
    Type(String),
}

#[derive(Debug, Clone)]
pub struct RIDData {
    rid: RID,
    scope: usize,
    line: usize,
    char_pos: usize,
}

#[derive(Debug, Clone)]
pub struct RIDStruct {
    ast: Vec<ASTStruct>,
    rid: Vec<RIDData>,
}

impl RIDStruct {
    pub fn new(ast: Vec<ASTStruct>) -> Self {
        Self {
            ast: ast.clone(),
            rid: vec![],
        }
    }
    pub(crate) fn match_cached(
        &mut self,
        cached: &mut Vec<ASTStruct>,
        cached_i: &mut usize,
        i: usize,
    ) {
        match cached.len() {
            1 => {
                // check if it's a byte
                if let AST::Byte(v) = cached[0].ast {
                    self.rid.push(RIDData {
                        rid: RID::Byte(v),
                        scope: cached[0].scope,
                        line: cached[0].line,
                        char_pos: cached[0].char_pos,
                    });
                    cached.clear();
                    *cached_i = i;
                }
            }
            2 => {
                if let AST::Byte(v) = cached[1].ast && let AST::Return = cached[0].ast {
                    self.rid.push(RIDData {
                        rid: RID::Return(Box::new(RID::Byte(v))),
                        scope: cached[1].scope,
                        line: cached[1].line,
                        char_pos: cached[1].char_pos,
                    });
                    cached.clear();
                    *cached_i = i;
                }
            }
            3 => {}
            _ => {
                if cached.len() >= 5 {
                    if let AST::Define = cached[0].ast
                        && let AST::Symbol(Symbol::SemiColon) = cached[cached.len() - 1].ast
                        && let AST::Name(name) = cached[1].ast.clone()
                        && cached[0].scope == cached[cached.len() - 1].scope
                        {
                            let len = cached.len();

                            let mut equal_sign_index = 0;
                            let mut found_equal = false;
                            let block = self
                                .ast
                                .clone()
                                .into_iter()
                                .zip(0..self.ast.clone().len())
                                .filter(|v| {
                                    info!("{found_equal}");
                                    if v.0.ast == AST::Symbol(Symbol::Equal) {
                                        found_equal = true;
                                        equal_sign_index = v.1;
                                        return false
                                    }
                                    if !found_equal {
                                        return false
                                    }
                                    true
                                })
                                .map(|x| x.0)
                                .collect::<Vec<_>>();
                            let args = cached[2..equal_sign_index]
                                .to_vec()
                                .iter()
                                .map(|x| {
                                    match x.ast.clone() {
                                        AST::Name(name) => (RID::Arg(name), x.clone()),
                                        _ => panic!("Args should be a name"),
                                    }
                                })
                                .collect::<Vec<_>>();
                            self.rid.push(RIDData {
                                rid: RID::FunctionDefined(name),
                                scope: cached[1].scope,
                                line: cached[1].line,
                                char_pos: cached[1].char_pos,
                            });
                            self.rid.push(RIDData {
                                rid: RID::FunctionDefinedArgsStart,
                                scope: cached[2].scope,
                                line: cached[2].line,
                                char_pos: cached[2].char_pos,
                            });
                            let mut ast_struct = cached[2].clone();
                            info!("{args:#?}");
                            args.iter().for_each(|x| {
                                self.rid.push(RIDData {
                                    rid: x.0.clone(),
                                    scope: x.1.scope,
                                    line: x.1.line,
                                    char_pos: x.1.char_pos
                                });
                                ast_struct = x.1.clone();
                            });
                            info!("{ast_struct:#?}");
                            self.rid.push(RIDData {
                                rid: RID::FunctionDefinedArgsEnd,
                                scope: ast_struct.clone().scope,
                                line: ast_struct.clone().line,
                                char_pos: ast_struct.clone().char_pos,
                            });
                            self.rid.push(RIDData {
                                rid: RID::FunctionDefinedContentsStart,
                                scope: cached[2 + args.len()].scope,
                                line: cached[2 + args.len()].line,
                                char_pos: cached[2 + args.len()].char_pos,
                            });
                            let mut block_cached = vec![];
                            let mut block_i = 0;
                            block.clone().into_iter().enumerate().for_each(|(i, x)| {
                                info!("Ran block");
                                self.match_cached(&mut block_cached, &mut block_i, i);
                                block_cached.push(x)
                            });
                            self.match_cached(&mut block_cached, &mut block_i, i);
                            self.rid.push(RIDData {
                                rid: RID::FunctionDefinedContentsEnd,
                                scope: cached[cached.len()-1].scope,
                                line: cached[cached.len()-1].line,
                                char_pos: cached[cached.len()-1].char_pos,
                            });
                        }
                }
            }
        }
    }
    pub fn run(mut self) -> Vec<RIDData> {
        let mut cached_i = 0;
        let mut cached: Vec<ASTStruct> = vec![];
        for (i, x) in self.ast.clone().into_iter().enumerate() {
            self.match_cached(&mut cached, &mut cached_i, i.clone());
            cached.push(x);
        }
        self.match_cached(&mut cached, &mut cached_i, self.ast.clone().len());
        return self.rid.clone();
    }
}
