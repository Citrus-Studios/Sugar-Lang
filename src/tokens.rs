#[derive(Debug, Clone, PartialEq)]
pub enum Tokens {
    Bang,
    Equal,
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
    Null,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokensStruct {
    pub token: Tokens,
    pub string: String,
    pub line: usize,
    pub char_pos: usize,
    pub scope: usize,
}

impl TokensStruct {
    pub fn new(token: Tokens, string: String, line: usize, char_pos: usize, scope: usize) -> Self {
        Self {
            token,
            string,
            line,
            char_pos,
            scope,
        }
    }
}

impl Into<Tokens> for String {
    fn into(self) -> Tokens {
        match self.as_str() {
            "!" => Tokens::Bang,
            "}" | "{" | "]" | "[" | ")" | "(" => Tokens::Delimiter,
            " " | "\t" => Tokens::Space,
            "+" => Tokens::Add,
            "*" => Tokens::Multiply,
            "-" => Tokens::Subtract,
            ";" => Tokens::SemiColon,
            ":" => Tokens::Colon,
            "\n" => Tokens::NewLine,
            "=" => Tokens::Equal,
            ">" => Tokens::Greater,
            "<" => Tokens::Less,
            _ => Tokens::Ident,
        }
    }
}
