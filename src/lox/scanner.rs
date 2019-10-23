use crate::lox::token::{Literal, Token, TokenType};
use phf::phf_map;
use std::char;
use std::iter::Peekable;
use std::str::Chars;

static SINGLE_TOKEN_MAP: phf::Map<char, TokenType> = phf_map! {
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

    fn string(&mut self) {
        let line_start = self.line;
        let mut word = Vec::new();

        while self.chars.peek() != Some(&'"') && self.chars.peek() != None {
            match self.chars.next().unwrap() {
                '\n' => self.line += 1,
                ' ' | '\t' | '\r' => (),
                next_char => word.push(next_char),
            }
        }

        match self.chars.peek() {
            Some(&'"') => {
                self.chars.next();
                self.add_token(
                    TokenType::String,
                    word.into_iter().collect::<String>(),
                    Literal::None,
                )
            }
            None => {
                println!("Unterminated string started at line: {}", line_start);
                self.errors.push(self.line);
            }

            _ => panic!(),
        }
    }

    fn scan_next_token(&mut self) {
        let ch = self.chars.next().unwrap_or(' ');
        if SINGLE_TOKEN_MAP.contains_key(&ch) {
            let token = SINGLE_TOKEN_MAP.get(&ch).unwrap().clone();
            self.add_token(token, &ch.to_string(), Literal::None)
        }
        match ch {
            '"' => self.string(),
            ' ' | '\t' | '\r' => (),
            '\n' => self.line += 1,
            '!' => {
                let next_ch = self.chars.peek();
                if next_ch == Some(&'=') {
                    self.add_token(TokenType::BangEqual, "!=", Literal::None);
                    self.chars.next();
                } else {
                    self.add_token(TokenType::Bang, &"!", Literal::None);
                }
            }
            '=' => {
                let next_ch = self.chars.peek();
                if next_ch == Some(&'=') {
                    self.add_token(TokenType::EqualEqual, "==", Literal::None);
                    self.chars.next();
                } else {
                    self.add_token(TokenType::Equal, "=", Literal::None);
                }
            }
            '<' => {
                let next_ch = self.chars.peek();
                if next_ch == Some(&'=') {
                    self.add_token(TokenType::LessEqual, "<=", Literal::None);
                    self.chars.next();
                } else {
                    self.add_token(TokenType::Less, "<", Literal::None);
                }
            }
            '>' => {
                let next_ch = self.chars.peek();
                if next_ch == Some(&'=') {
                    self.add_token(TokenType::GreaterEqual, ">=", Literal::None);
                    self.chars.next();
                } else {
                    self.add_token(TokenType::Greater, "<", Literal::None);
                }
            }
            '/' => {
                let next_ch = self.chars.peek();
                if next_ch == Some(&'/') {
                    let mut next = self.chars.next();
                    while next != Some('\n') && next != None {
                        next = self.chars.next();
                    }
                } else {
                    self.add_token(TokenType::Slash, "/", Literal::None);
                }
            }
            _ => {
                println!("Unexpected char at line: {}", self.line);
                self.errors.push(self.line);
            }
        }
    }

    fn add_token<S: AsRef<str>>(&mut self, token: TokenType, token_str: S, lit: Literal) {
        let next_token = Token::new(token, token_str.as_ref().to_string(), lit, self.line as u32);
        self.tokens.push(next_token);
    }
}
