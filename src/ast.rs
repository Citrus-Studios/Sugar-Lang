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
    pub fn get_block(&mut self) -> Result<&mut AST, String> {
        match self.ast {
            AST::Block { .. } => Ok(&mut self.ast),
            _ => Err("Couldn't get block".to_string()),
        }
    }
}
