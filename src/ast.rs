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
    Symbol(Symbol),
}

pub struct ASTStruct {
    pub ast: AST,
    pub char_pos: u128,
    pub line: u128,
}

pub enum Symbol {
    Equal,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Equality,
    Bang,
    Arrow,
    SemiColon,
    Colon,
}

impl ASTStruct {
    pub fn get_block(&mut self) -> Result<(&mut usize, &mut Vec<ASTStruct>), String> {
        match self.ast {
            AST::Block { scope, contents } => Ok((&mut scope, &mut contents)),
            _ => Err("Couldn't get block".to_string()),
        }
    }
}
