use self::expr::{Binary, Expr, Unary};
use self::token::TokenType;
struct Parser {
    token_list: Peekable<Token<'a>>,
    current: Token,
}

impl Parser {
    fn new(token_list: Vec<Token>) -> Parser {
        Parser { token_list }
    }

    pub fn parse(&mut self) {
        // Does stuff
    }

    fn expression(&mut self) -> Expr {
        equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = comparison();

        let peek = self.token_list.peek();

        while let Token::BangEqual | Token::EqualEqual = peek {
            operator = self.token_list.next();
            let right = comparison();
            expr = Binary {
                left: expr,
                right,
                operator,
            }
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = addition();

        let peek = self.token_list.peek();

        while let Token::Greater | Token::GreaterEqual | Token::Less | Token::LessEqual = peek {
            operator = self.token_list.next();
            let right = multiplication();
            expr = Binary {
                left: expr,
                right,
                operator,
            }
        }

        expr
    }

    fn addition(&mut self) -> Expr {
        let mut expr = multiplication();

        let peek = self.token_list.peek();

        while let Token::Minus | Token::Plus = peek {
            operator = self.token_list.next();
            let right = unary();
            expr = Binary {
                left: expr,
                right,
                operator,
            }
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        let peek = self.token_list.peek();

        if let Token::Bang | Token::Minus = peek {
            operator = self.token_list.next();
            let right = unary();
            Unary { right, operator }
        } else {
            primary
        }
    }

    fn primary(&mut self) -> Expr {
        let peek = self.token_list.peek();
    }
}
