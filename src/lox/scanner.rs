use super::token::{Literal, Token, TokenType};
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

static KEYWORDS: phf::Map<&str, TokenType> = phf_map! {
    "and" =>    TokenType::And,
    "class" =>  TokenType::Class,
    "else" =>   TokenType::Else,
    "false" =>  TokenType::False,
    "for" =>    TokenType::For,
    "fun" =>    TokenType::Fun,
    "if" =>     TokenType::If,
    "nil" =>    TokenType::Nil,
    "or" =>     TokenType::Or,
    "print" =>  TokenType::Print,
    "return" => TokenType::Return,
    "super" =>  TokenType::Super,
    "this" =>   TokenType::This,
    "true" =>   TokenType::True,
    "var" =>    TokenType::Var,
    "while" =>  TokenType::While,
};

fn key_getter(identifier: &str) -> (bool, &str) {
    let mut contain_key = false;
    let mut keyword_key = "";
    for key in KEYWORDS.keys() {
        if key == &identifier {
            contain_key = true;
            keyword_key = key;
        }
    }
    (contain_key, keyword_key)
}

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

    pub fn scan_text(&mut self) -> &Vec<Token> {
        while self.chars.peek() != None {
            self.scan_next_token();
        }
        self.tokens.push(Token::empty_token(self.line as u32));

        &self.tokens
    }

    fn identifier(&mut self, first_digit: char) {
        let mut identifier = String::new();
        identifier.push(first_digit);
        while self.chars.peek() != Some(&' ')
            && self.chars.peek() != Some(&';')
            && self.chars.peek() != None
        {
            if SINGLE_TOKEN_MAP.contains_key(self.chars.peek().unwrap()) {
                break;
            }
            match self.chars.peek().unwrap() {
                'a'..='z' | 'A'..='Z' | '_' | '-' => {
                    identifier.push(self.chars.next().unwrap());
                }
                ch => {
                    println!("Unsupported char {} at line {}", ch, self.line);
                    self.chars.next();
                    self.errors.push(self.line);
                }
            }
        }
        let (contain_key, keyword_key) = key_getter(&identifier);
        if contain_key {
            let t_type = KEYWORDS.get(keyword_key).unwrap();
            match t_type {
                TokenType::True => self.add_token(*t_type, identifier, Literal::Boolean(true)),
                TokenType::False => self.add_token(*t_type, identifier, Literal::Boolean(false)),
                TokenType::Nil => self.add_token(*t_type, identifier, Literal::Nil),
                _ => self.add_token(*t_type, identifier, Literal::None),
            }
        } else {
            self.add_token(TokenType::Identifier, identifier, Literal::None);
        }
    }

    fn number(&mut self, first_digit: char) {
        let mut number = String::new();
        number.push(first_digit);

        while self.chars.peek() != Some(&' ') && self.chars.peek() != None {
            let chr = self.chars.peek().unwrap();
            if chr == &'.' {
                self.chars.next().unwrap();
                number.push('.');
                if self.chars.peek().unwrap_or(&' ').is_digit(10) {
                    number.push(self.chars.next().unwrap());
                } else {
                    println!("Unterminated number at line: {}", self.line);
                    self.errors.push(self.line);
                }
            } else if chr.is_digit(10) {
                let digit = self.chars.next().unwrap();
                number.push(digit);
            } else {
                let ch = self.chars.peek().unwrap();
                if SINGLE_TOKEN_MAP.contains_key(&ch) {
                    break;
                } else {
                    println!("Unterminated number started at line: {}", self.line);
                    self.chars.next();
                    self.errors.push(self.line);
                }
            }
        }
        let parsed_number = number.parse::<f64>().unwrap();
        self.add_token(TokenType::Number, number, Literal::F64(parsed_number))
    }

    fn string(&mut self) {
        let line_start = self.line;
        let mut word = Vec::new();

        while self.chars.peek() != Some(&'"') && self.chars.peek() != None {
            match self.chars.next().unwrap() {
                '\n' => self.line += 1,
                next_char => word.push(next_char),
            }
        }

        match self.chars.peek() {
            Some(&'"') => {
                self.chars.next();
                let word_clone = word.clone();
                self.add_token(
                    TokenType::String,
                    word.into_iter().collect::<String>(),
                    Literal::String(word_clone.into_iter().collect::<String>()),
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
            let token = SINGLE_TOKEN_MAP.get(&ch).unwrap();
            return self.add_token(*token, &ch.to_string(), Literal::None);
        }
        match ch {
            '"' => self.string(),
            '0'..='9' => self.number(ch),
            'a'..='z' | 'A'..='Z' | '_' => self.identifier(ch),
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
                    self.add_token(TokenType::Greater, ">", Literal::None);
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
            char => {
                println!("Unexpected char {:?} at line: {}", char, self.line);
                self.errors.push(self.line);
            }
        }
    }

    fn add_token<S: AsRef<str>>(&mut self, token: TokenType, token_str: S, lit: Literal) {
        let next_token = Token::new(token, token_str.as_ref().to_string(), lit, self.line as u32);
        self.tokens.push(next_token);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lox::token::Literal;

    #[test]
    fn literal_string() {
        let string_token = Token::new(
            TokenType::String,
            "string".to_owned(),
            Literal::String("string".into()),
            1,
        );

        let semicolon = Token::new(TokenType::Semicolon, ";".to_owned(), Literal::None, 1);
        let eof = Token::new(TokenType::EOF, "".to_owned(), Literal::None, 1);

        let tokens = vec![string_token, semicolon, eof];

        let text = "\"string\";";
        let mut scanner = Scanner::new(text);
        assert_eq!(tokens, *scanner.scan_text());
    }

    #[test]
    fn negation() {
        let number = Token::new(TokenType::Number, "1".to_owned(), Literal::F64(1.0), 1);
        let operator = Token::new(TokenType::Bang, "!".to_owned(), Literal::None, 1);

        let semicolon = Token::new(TokenType::Semicolon, ";".to_owned(), Literal::None, 1);
        let eof = Token::new(TokenType::EOF, "".to_owned(), Literal::None, 1);
        let tokens = vec![operator, number, semicolon, eof];

        let text = "!1;";
        let mut scanner = Scanner::new(text);
        assert_eq!(tokens, *scanner.scan_text());
    }

    #[test]
    fn addition() {
        let first_number = Token::new(TokenType::Number, "1".to_owned(), Literal::F64(1.0), 1);
        let second_number = Token::new(TokenType::Number, "2".to_owned(), Literal::F64(2.0), 1);
        let operator = Token::new(TokenType::Plus, "+".to_owned(), Literal::None, 1);

        let semicolon = Token::new(TokenType::Semicolon, ";".to_owned(), Literal::None, 1);
        let eof = Token::new(TokenType::EOF, "".to_owned(), Literal::None, 1);
        let tokens = vec![first_number, operator, second_number, semicolon, eof];

        let text = "1 + 2;";
        let mut scanner = Scanner::new(text);
        assert_eq!(tokens, *scanner.scan_text());
    }

    #[test]
    fn equality() {
        let first_number = Token::new(TokenType::Number, "1".to_owned(), Literal::F64(1.0), 1);
        let second_number = Token::new(TokenType::Number, "1".to_owned(), Literal::F64(1.0), 1);
        let operator = Token::new(TokenType::EqualEqual, "==".to_owned(), Literal::None, 1);

        let semicolon = Token::new(TokenType::Semicolon, ";".to_owned(), Literal::None, 1);
        let eof = Token::new(TokenType::EOF, "".to_owned(), Literal::None, 1);
        let tokens = vec![first_number, operator, second_number, semicolon, eof];

        let text = "1 == 1;";
        let mut scanner = Scanner::new(text);
        assert_eq!(tokens, *scanner.scan_text());
    }

    #[test]
    fn comparison() {
        let first_number = Token::new(TokenType::Number, "1".to_owned(), Literal::F64(1.0), 1);
        let second_number = Token::new(TokenType::Number, "2".to_owned(), Literal::F64(2.0), 1);
        let operator = Token::new(TokenType::Greater, ">".to_owned(), Literal::None, 1);
        let semicolon = Token::new(TokenType::Semicolon, ";".to_owned(), Literal::None, 1);

        let eof = Token::new(TokenType::EOF, "".to_owned(), Literal::None, 1);
        let tokens = vec![first_number, operator, second_number, semicolon, eof];

        let text = "1 > 2;";
        let mut scanner = Scanner::new(text);
        assert_eq!(tokens, *scanner.scan_text());
    }

    #[test]
    fn multiplication() {
        let first_number = Token::new(TokenType::Number, "1".to_owned(), Literal::F64(1.0), 1);
        let operator = Token::new(TokenType::Star, "*".to_owned(), Literal::None, 1);
        let second_number = Token::new(TokenType::Number, "1".to_owned(), Literal::F64(1.0), 1);
        let semicolon = Token::new(TokenType::Semicolon, ";".to_owned(), Literal::None, 1);

        let eof = Token::new(TokenType::EOF, "".to_owned(), Literal::None, 1);
        let tokens = vec![first_number, operator, second_number, semicolon, eof];
        let text = "1 * 1;";
        let mut scanner = Scanner::new(text);
        assert_eq!(tokens, *scanner.scan_text());
    }

    #[test]
    fn while_statement() {
        let while_kw = Token::new(TokenType::While, "while".to_owned(), Literal::None, 1);

        let left_paren = Token::new(TokenType::LeftParen, "(".to_owned(), Literal::None, 1);
        let variable = Token::new(TokenType::Identifier, "a".to_owned(), Literal::None, 1);
        let greater = Token::new(TokenType::Less, "<".to_owned(), Literal::None, 1);
        let two = Token::new(TokenType::Number, "2".to_owned(), Literal::F64(2.0), 1);
        let right_paren = Token::new(TokenType::RightParen, ")".to_owned(), Literal::None, 1);

        let left_bracket = Token::new(TokenType::LeftBrace, "{".to_owned(), Literal::None, 1);
        let plus_sign = Token::new(TokenType::Plus, "+".to_owned(), Literal::None, 1);
        let equal_sign = Token::new(TokenType::Equal, "=".to_owned(), Literal::None, 1);
        let one = Token::new(TokenType::Number, "1".to_owned(), Literal::F64(1.0), 1);
        let right_bracket = Token::new(TokenType::RightBrace, "}".to_owned(), Literal::None, 1);
        let semicolon = Token::new(TokenType::Semicolon, ";".to_owned(), Literal::None, 1);

        let eof = Token::new(TokenType::EOF, "".to_owned(), Literal::None, 1);
        let tokens = vec![
            while_kw,
            left_paren,
            variable.clone(),
            greater.clone(),
            two.clone(),
            right_paren,
            left_bracket,
            variable.clone(),
            equal_sign,
            variable.clone(),
            plus_sign,
            one,
            semicolon,
            right_bracket,
            eof,
        ];

        let text = "while(a < 2) { a = a + 1; }";
        let mut scanner = Scanner::new(text);
        assert_eq!(tokens, *scanner.scan_text());
    }
}
