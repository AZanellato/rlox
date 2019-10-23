use crate::lox::token::{Literal, Token, TokenType};
use phf::phf_map;
use std::char;
use std::iter::Peekable;
use std::str::Chars;

static SINGLE_TOKEN_MAP: phf::Map<char, TokenType> = phf_map! {
    '<' => TokenType::Greater,
    '/' => TokenType::Slash,
    '(' => TokenType::LeftParen,
    ')' => TokenType::RightParen,
    '{' => TokenType::LeftBrace,
    '}' => TokenType::RightBrace,
    ',' => TokenType::Comma,
    '.' => TokenType::Dot,
    '-' => TokenType::Minus,
    '+' => TokenType::Plus,
    ';' => TokenType::Semicolon,
    '*' => TokenType::Star,
    '!' => TokenType::Bang,
    '=' => TokenType::Equal,
};

pub struct Scanner<'a> {
    chars: Peekable<Chars<'a>>,
    tokens: Vec<Token>,
    errors: Vec<usize>,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &str) -> Scanner {
        Scanner {
            chars: source.chars().peekable(),
            tokens: vec![],
            errors: vec![],
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while self.chars.peek() != None {
            self.scan_next_token();
        }
        let next_token = Token::new(
            TokenType::EOF,
            "".to_owned(),
            Literal::None,
            self.line as u32,
        );
        self.tokens.push(next_token);
        &self.tokens
    }

    fn scan_next_token(&mut self) {
        let ch = self.chars.next();

        match ch {
            Some('(') => {
                self.add_token(TokenType::LeftParen, &"(", Literal::None);
            }
            Some(')') => {
                self.add_token(TokenType::RightParen, &")", Literal::None);
            }
            Some('{') => {
                self.add_token(TokenType::LeftBrace, &"{", Literal::None);
            }
            Some('}') => {
                self.add_token(TokenType::RightBrace, &"}", Literal::None);
            }
            Some(',') => {
                self.add_token(TokenType::Comma, &",", Literal::None);
            }
            Some('.') => {
                self.add_token(TokenType::Dot, &".", Literal::None);
            }
            Some('-') => {
                self.add_token(TokenType::Minus, &"-", Literal::None);
            }
            Some('+') => {
                self.add_token(TokenType::Plus, &"+", Literal::None);
            }
            Some(';') => {
                self.add_token(TokenType::Semicolon, &";", Literal::None);
            }
            Some('*') => {
                self.add_token(TokenType::Star, &"*", Literal::None);
            }
            Some('!') => {
                let next_ch = self.chars.peek();
                if next_ch == Some(&'=') {
                    self.add_token(TokenType::BangEqual, "!=", Literal::None);
                    self.chars.next();
                } else {
                    self.add_token(TokenType::Bang, &"!", Literal::None);
                }
            }
            Some('=') => {
                let next_ch = self.chars.peek();
                if next_ch == Some(&'=') {
                    self.add_token(TokenType::EqualEqual, "==", Literal::None);
                    self.chars.next();
                } else {
                    self.add_token(TokenType::Equal, "=", Literal::None);
                }
            }
            Some('<') => {
                let next_ch = self.chars.peek();
                if next_ch == Some(&'=') {
                    self.add_token(TokenType::LessEqual, "<=", Literal::None);
                    self.chars.next();
                } else {
                    self.add_token(TokenType::Less, "<", Literal::None);
                }
            }
            Some('>') => {
                let next_ch = self.chars.peek();
                if next_ch == Some(&'=') {
                    self.add_token(TokenType::GreaterEqual, ">=", Literal::None);
                    self.chars.next();
                } else {
                    self.add_token(TokenType::Greater, "<", Literal::None);
                }
            }
            Some('/') => {
                let next_ch = self.chars.peek();
                if next_ch == Some(&'/') {
                    let mut next = self.chars.next();
                    while next != Some('\n') || next != None {
                        next = self.chars.next();
                    }
                } else {
                    self.add_token(TokenType::Slash, "/", Literal::None);
                }
            }
            Some('\n') => self.line += 1,
            None | Some(' ') | Some('\t') | Some('\r') => (),
            Some(_) => {
                println!("Unexpected char at line: {}", self.line);
                self.errors.push(self.line);
            }
        }
    }

    fn add_token(&mut self, token: TokenType, token_str: &str, lit: Literal) {
        let next_token = Token::new(token, token_str.to_owned(), lit, self.line as u32);
        self.tokens.push(next_token);
    }
}
