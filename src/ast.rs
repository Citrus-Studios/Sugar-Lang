use core::fmt;
use std::fmt::Debug;

#[derive(PartialEq, Clone)]
pub enum AST {
    Byte(u8),
    Return,
    Define,
    Declare,
    Type(String),
    Name(String),
    Symbol(Symbol),
}

impl Debug for AST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AST::Byte(x) => format!("Byte: {x}"),
                AST::Return => "Return".to_string(),
                AST::Define => "Define".to_string(),
                AST::Declare => "Declare".to_string(),
                AST::Type(x) => format!("Type: {x}"),
                AST::Name(x) => format!("Name: {x}"),
                AST::Symbol(x) => format!("Symbol: {x:?}"),
            }
        )
    }
}

#[derive(Clone)]
pub struct ASTStruct {
    pub ast: AST,
    pub scope: usize,
    pub char_pos: usize,
    pub line: usize,
}

impl Debug for ASTStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let tabs = (0..self.scope)
            .map(|_| "↳  ".to_string())
            .collect::<String>();
        write!(
            f,
            "{}{:#?} | {}C {}L",
            tabs, self.ast, self.char_pos, self.line
        )
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Symbol {
    Equal,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    InEquality,
    Equality,
    Bang,
    Arrow,
    SemiColon,
    Colon,
}
