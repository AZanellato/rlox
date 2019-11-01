use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    F64(f64),
    None,
}

#[derive(Debug, Clone)]
pub struct Token {
    token: TokenType,
    pub lexeme: String,
    literal: Literal,
    line: u32,
}

impl Token {
    pub fn new(token: TokenType, lexeme: String, literal: Literal, line: u32) -> Self {
        Token {
            token,
            lexeme,
            literal,
            line,
        }
    }

    pub fn to_string(&self) {
        format!("{:?} {:?} {:?}", self.token, self.lexeme, self.literal);
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {:?} {:?}", self.token, self.lexeme, self.literal)
    }
}
