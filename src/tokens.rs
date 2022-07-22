pub enum Tokens {
    Bang,
    Char,
    Equal,
    Minus,
    Add,
    Multiply,
    Divide,
    Delimiter,
    SemiColon,
    Colon,
    Space,
}

pub struct TokensStruct {
    token: Tokens,
    char: char,
    line: u128,
    char_pos: u128,
}

impl TokensStruct {
    pub fn new(token: Tokens, char: char, line: u128, char_pos: u128) -> Self {
        Self {
            token,
            char,
            line,
            char_pos,
        }
    }
}
