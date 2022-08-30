use plex::lexer;

#[derive(Debug, Clone)]
pub enum Token {
    Ident(String),

    Variable,
    Mutate,

    Define,
    Declare,
    Return,

    Byte(u8),

    Bang,

    Equals,
    Plus,
    Minus,
    Star,
    Slash,

    Ampersand,
    Pipe,

    Percent,

    Gt,
    Lt,

    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,

    SemiColon,
    Colon,

    Whitespace,
    Comment,

    If,
    Else,
    For,

    Pass,
}

lexer! {
    fn next_token(text: 'a) -> Token;

    // "C-style" comments (/* .. */) - can't contain "*/"
    r#"/[*](~(.*[*]/.*))[*]/"# => Token::Comment,
    // "C++-style" comments (// ...)
    r#"//[^\n]*"# => Token::Comment,

    r#"[ \t\r\n]"# => Token::Whitespace,

    r#"var"# => Token::Variable,
    r#"mutate"# => Token::Mutate,

    r#"[0-9]+"# => Token::Byte(text.parse().unwrap()),

    r#"declare"# => Token::Declare,
    r#"define"# => Token::Define,
    r#"return"# => Token::Return,

    r#"if"# => Token::If,
    r#"else"# => Token::Else,

    r#"for"# => Token::For,

    r#"pass"# => Token::Pass,

    r#"[a-zA-Z_][a-zA-Z0-9_]*"# => Token::Ident(text.to_owned()),

    r#"="# => Token::Equals,
    r#"\+"# => Token::Plus,
    r#"-"# => Token::Minus,
    r#"\*"# => Token::Star,
    r#"/"# => Token::Slash,

    r#">"# => Token::Gt,
    r#"<"# => Token::Lt,

    r#"\|"# => Token::Pipe,
    r#"\&"# => Token::Ampersand,
    r#"%"# => Token::Percent,

    r#"\("# => Token::LParen,
    r#"\)"# => Token::RParen,
    r#"\["# => Token::LBracket,
    r#"\]"# => Token::RBracket,
    r#"\{"# => Token::LBrace,
    r#"\}"# => Token::RBrace,

    r#"\;"# => Token::SemiColon,
    r#"\:"# => Token::Colon,

    r#"!"# => Token::Bang,
}

pub struct Lexer<'a> {
    original: &'a str,
    remaining: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(s: &'a str) -> Lexer<'a> {
        Lexer {
            original: s,
            remaining: s,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub lo: usize,
    pub hi: usize,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (Token, Span);
    fn next(&mut self) -> Option<(Token, Span)> {
        loop {
            let (tok, span) = if let Some((tok, new_remaining)) = next_token(self.remaining) {
                let lo = self.original.len() - self.remaining.len();
                let hi = self.original.len() - new_remaining.len();
                self.remaining = new_remaining;
                (tok, Span { lo, hi })
            } else {
                return None;
            };
            match tok {
                Token::Whitespace | Token::Comment => {
                    continue;
                }
                tok => {
                    return Some((tok, span));
                }
            }
        }
    }
}
