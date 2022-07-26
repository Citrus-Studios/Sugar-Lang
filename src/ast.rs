use std::rc::Rc;

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
        self: Rc<Self>,
    ) -> Result<(&'a mut usize, &'a mut Vec<ASTStruct>), String> {
        match self.clone().ast {
            AST::Block {
                mut scope,
                mut contents,
            } => Ok((&mut scope, &mut contents)),
            _ => Err("Couldn't get block".to_string()),
        }
    }
}
