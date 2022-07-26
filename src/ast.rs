pub enum AST {
    Block { scope: usize, contents: Vec<AST<T>> },
    Byte(u8),
    Return,
    Define,
    Declare,
    Type(String),
    Symbol(Symbol),
}

pub struct ASTStruct {
    ast: AST,
    char_pos: u128,
    line: u128,
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
