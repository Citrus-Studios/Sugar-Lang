pub enum Tokens {
    Bang,
    Equal,
    Minus,
    Add,
    Multiply,
    Subtract,
    Divide,
    Delimiter,
    SemiColon,
    Colon,
    Space,
    NewLine,
    Ident,
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

impl Into<Tokens> for String {
    fn into(self) -> Tokens {
        match self.as_str() {
            "!" => Tokens::Bang,
            "}" | "{" | "]" | "[" | ")" | "(" => Tokens::Delimiter,
            " " => Tokens::Space,
            "+" => Tokens::Add,
            "*" => Tokens::Multiply,
            "-" => Tokens::Subtract,
            ";" => Tokens::SemiColon,
            ":" => Tokens::Colon,
            "\n" => Tokens::NewLine,
            _ => Tokens::Ident,
        }
    }
}
