#[derive(Clone)]
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

#[derive(Clone)]
pub struct ASTStruct {
    pub ast: AST,
    pub char_pos: u128,
    pub line: u128,
}

#[derive(Clone)]
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
    pub fn get_block<'a>(
        self,
    ) -> (
        Self,
        Result<(&'a mut usize, &'a mut Vec<ASTStruct>), String>,
    ) {
        match self.ast {
            AST::Block {
                mut scope,
                mut contents,
            } => (self, Ok((&mut scope, &mut contents))),
            _ => (self, Err("Couldn't get block".to_string())),
        }
    }
}
