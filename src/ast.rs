#[derive(Debug, Clone)]
pub enum AST {
    Block {
        scope: usize,
        contents: Vec<ASTStruct>,
    },
    Byte(u8),
    Return,
    Define,
    Declare,
    Type(String),
    Name(String),
    Symbol(Symbol),
}

#[derive(Debug, Clone)]
pub struct ASTStruct {
    pub ast: AST,
    pub char_pos: u128,
    pub line: u128,
}

#[derive(Debug, Clone)]
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

impl ASTStruct {
    pub fn get_block<'a>(&'a mut self) -> Result<(&'a mut usize, &'a mut Vec<ASTStruct>), String> {
        let ast: &'a mut AST = &mut self.ast;
        match ast {
            AST::Block { scope, contents } => Ok((scope, contents)),
            _ => Err("Couldn't get block".to_string()),
        }
    }
}
