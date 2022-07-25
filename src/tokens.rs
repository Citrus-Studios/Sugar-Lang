#[derive(Debug, Clone, PartialEq)]
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
    Greater,
    Less,
    Ident,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokensStruct {
    pub token: Tokens,
    pub string: String,
    pub line: u128,
    pub char_pos: u128,
}

impl TokensStruct {
    pub fn new(token: Tokens, string: String, line: u128, char_pos: u128) -> Self {
        Self {
            token,
            string,
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
            ">" => Tokens::Greater,
            "<" => Tokens::Less,
            _ => Tokens::Ident,
        }
    }
}
