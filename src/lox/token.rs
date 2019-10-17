#[derive(Debug)]
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

#[derive(Debug)]
pub enum Literal {
    String,
    f64,
    None,
}

#[derive(Debug)]
pub struct Token {
    token: TokenType,
    lexeme: String,
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
