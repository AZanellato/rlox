use std::fmt;

#[derive(PartialEq, Debug, Copy, Clone)]
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

#[derive(PartialEq, Debug, Clone)]
pub enum Literal {
    String(String),
    F64(f64),
    Boolean(bool),
    Nil,
    None,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    pub t_type: TokenType,
    pub lexeme: String,
    pub literal: Literal,
    pub line: u32,
}

impl Token {
    pub fn new(t_type: TokenType, lexeme: String, literal: Literal, line: u32) -> Self {
        Token {
            t_type,
            lexeme,
            literal,
            line,
        }
    }

    pub fn empty_token(line: u32) -> Self {
        Self {
            t_type: TokenType::EOF,
            lexeme: "".to_owned(),
            literal: Literal::None,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {:?} {:?}", self.t_type, self.lexeme, self.literal)
    }
}
