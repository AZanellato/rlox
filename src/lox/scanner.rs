use crate::lox::token::{Literal, Token, TokenType};

pub struct Scanner {
    source: Vec<u8>,
    tokens: Vec<Token>,
    errors: Vec<usize>,
    current_pos: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source: source.as_bytes().to_owned(),
            tokens: vec![],
            errors: vec![],
            current_pos: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        let total_size = self.source.len();
        let start = 0;
        let line = 1;
        // self.source.split_whitespace()
        while total_size > self.current_pos {
            // We are at the beginning of the next lexeme.
            let start = self.current_pos;
            self.scan_token(line);
        }
        let next_token = Token::new(TokenType::EOF, "".to_owned(), Literal::None, line as u32);
        self.tokens.push(next_token);
        &self.tokens
    }

    fn scan_token(&mut self, line: usize) {
        let ch = self.source.get(self.current_pos);
        let next_ch = self.source.get(self.current_pos + 1);
        match ch {
            Some(b'(') => {
                self.add_token(TokenType::LeftParen, &"(", Literal::None, line);
                self.current_pos += 1
            }
            Some(b')') => {
                self.add_token(TokenType::RightParen, &")", Literal::None, line);
                self.current_pos += 1
            }
            Some(b'{') => {
                self.add_token(TokenType::LeftBrace, &"{", Literal::None, line);
                self.current_pos += 1
            }
            Some(b'}') => {
                self.add_token(TokenType::RightBrace, &"}", Literal::None, line);
                self.current_pos += 1
            }
            Some(b',') => {
                self.add_token(TokenType::Comma, &",", Literal::None, line);
                self.current_pos += 1
            }
            Some(b'.') => {
                self.add_token(TokenType::Dot, &".", Literal::None, line);
                self.current_pos += 1
            }
            Some(b'-') => {
                self.add_token(TokenType::Minus, &"-", Literal::None, line);
                self.current_pos += 1
            }
            Some(b'+') => {
                self.add_token(TokenType::Plus, &"+", Literal::None, line);
                self.current_pos += 1
            }
            Some(b';') => {
                self.add_token(TokenType::Semicolon, &";", Literal::None, line);
                self.current_pos += 1
            }
            Some(b'*') => {
                self.add_token(TokenType::Star, &"*", Literal::None, line);
                self.current_pos += 1
            }
            Some(b'!') => {
                if next_ch == Some(&b'=') {
                    self.add_token(TokenType::BangEqual, "!=", Literal::None, line);
                    self.current_pos += 2
                } else {
                    self.add_token(TokenType::Bang, &"!", Literal::None, line);
                    self.current_pos += 1
                }
            }
            Some(b'=') => {
                if next_ch == Some(&b'=') {
                    self.add_token(TokenType::EqualEqual, "==", Literal::None, line);
                    self.current_pos += 2
                } else {
                    self.add_token(TokenType::Equal, "=", Literal::None, line);
                    self.current_pos += 1
                }
            }
            Some(b'<') => {
                if next_ch == Some(&b'=') {
                    self.add_token(TokenType::LessEqual, "<=", Literal::None, line);
                    self.current_pos += 2
                } else {
                    self.add_token(TokenType::Less, "<", Literal::None, line);
                    self.current_pos += 1
                }
            }
            Some(b'>') => {
                if next_ch == Some(&b'=') {
                    self.add_token(TokenType::GreaterEqual, ">=", Literal::None, line);
                    self.current_pos += 2
                } else {
                    self.add_token(TokenType::Greater, "<", Literal::None, line);
                    self.current_pos += 1
                }
            }
            Some(b'/') => {
                if next_ch == Some(&b'/') {
                    let peek = self.source.get(self.current_pos + 1);
                    while peek != Some(&b'\n') || peek != None {
                        self.current_pos += 1;
                    }
                } else {
                    self.add_token(TokenType::Slash, "/", Literal::None, line);
                    self.current_pos += 1
                }
            }
            Some(b' ') | Some(b'\n') | Some(b'\t') | Some(b'\r') => self.current_pos += 1,
            Some(_) => {
                println!("Unexpected char at line: {}", line);
                self.errors.push(line);
                self.current_pos += 1
            }
            None => self.current_pos += 1,
        }
    }

    fn add_token(&mut self, token: TokenType, token_str: &str, lit: Literal, line: usize) {
        let next_token = Token::new(token, token_str.to_owned(), lit, line as u32);
        self.tokens.push(next_token);
    }
}
