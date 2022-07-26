pub enum AST<T> {
    Block { scope: usize, contents: Vec<AST<T>> },
    Literal(T),
    Return,
    Define,
    Declare,
    Type(String),
    Symbol(Symbol),
}

pub struct ASTStruct<T> {
    ast: AST<T>,
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
