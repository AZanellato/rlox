use super::expr::{Assignment, Binary, Expr, Grouping, Literal, Logical, Unary, Var};
use super::stmt::{self, Block, IfStmt, Stmt, While};
use super::token::{self, Token, TokenType};
use std::iter::Peekable;
use std::slice::Iter;

#[derive(Debug)]
pub struct Parser<'a> {
    token_list: Peekable<Iter<'a, Token>>,
    error: bool,
}

impl<'a> Parser<'a> {
    pub fn new(borrowed_token_list: &'a [Token]) -> Self {
        Parser {
            token_list: borrowed_token_list.iter().peekable(),
            error: false,
        }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while let Some(parsed_expression) = self.next_stmt() {
            statements.push(parsed_expression);
        }

        statements
    }

    fn next_stmt(&mut self) -> Option<Stmt> {
        let peek_token = self.token_list.peek()?;

        match peek_token.t_type {
            TokenType::Print => {
                self.token_list.next();
                self.print_statement()
            }
            TokenType::Var => {
                self.token_list.next();
                self.variable_declaration()
            }
            TokenType::LeftBrace => {
                self.token_list.next();
                self.block_statement()
            }
            TokenType::For => {
                self.token_list.next();
                self.for_loop()
            }
            TokenType::If => {
                self.token_list.next();
                self.if_statement()
            }
            TokenType::While => {
                self.token_list.next();
                self.while_statement()
            }
            _ => self.stmt_expr(),
        }
    }

    fn print_statement(&mut self) -> Option<Stmt> {
        let value = self.expression();
        let next_token = self.token_list.peek();
        if next_token == None || value == None {
            return None;
        }

        if next_token?.t_type != TokenType::Semicolon {
            println!("Expect ; after value")
        }
        self.token_list.next();

        Some(Stmt::Print(value.unwrap()))
    }

    fn variable_declaration(&mut self) -> Option<Stmt> {
        let name = self.token_list.next()?;

        let next_token = self.token_list.peek();
        if next_token == None {
            println!("Expect ; after value");
            return None;
        }
        let next_token = next_token.unwrap();

        if next_token.t_type == TokenType::Semicolon {
            return Some(self.empty_init(name));
        }

        if next_token.t_type != TokenType::Equal {
            println!("Expected = or ; after var");
            return None;
        }

        let equal_token = self.token_list.next();

        let expr_value = self.expression();

        let value = if expr_value == None {
            let token = Token::new(
                TokenType::Nil,
                "".to_owned(),
                token::Literal::None,
                equal_token?.line,
            );
            let literal = Literal { token };

            Expr::Literal(literal)
        } else {
            expr_value.unwrap()
        };

        let variable = stmt::Var {
            value,
            name: name.lexeme.to_owned(),
        };

        let next_token = self.token_list.next();

        if let Some(token) = next_token {
            if token.t_type == TokenType::Semicolon {
                return Some(Stmt::Declaration(variable));
            }
        }
        panic!("Expect ; after declaration");
    }

    fn empty_init(&mut self, name: &Token) -> Stmt {
        let token = Token::new(
            TokenType::Nil,
            "".to_owned(),
            token::Literal::None,
            name.line,
        );
        let literal = Literal { token };
        let value = Expr::Literal(literal);

        let variable = stmt::Var {
            value,
            name: name.lexeme.to_owned(),
        };

        Stmt::Declaration(variable)
    }

    fn block_statement(&mut self) -> Option<Stmt> {
        let mut statements = vec![];

        while let Some(next_token) = self.token_list.peek() {
            if next_token.t_type == TokenType::RightBrace {
                break;
            }

            let next_stmt = self.next_stmt()?;
            statements.push(next_stmt);
        }

        if self.token_list.peek() == None {
            panic!("Missing closing bracket");
        }

        if self.token_list.peek()?.t_type == TokenType::RightBrace {
            self.token_list.next();
        }

        Some(Stmt::Block(Block {
            stmt_vec: statements,
        }))
    }

    fn for_loop(&mut self) -> Option<Stmt> {
        let next = self.token_list.next()?;
        if next.t_type != TokenType::LeftParen {
            panic!("Expect `(` after `for`.");
        }

        let peek = self.token_list.peek()?;
        let initializer = match peek.t_type {
            TokenType::Semicolon => None,
            TokenType::Var => {
                self.token_list.next();
                self.variable_declaration()
            }
            _ => self.stmt_expr(),
        };
        let peek = self.token_list.peek();

        let written_condition = match peek {
            None => None,
            Some(token) => match token.t_type {
                TokenType::Semicolon => None,
                _ => self.expression(),
            },
        };
        let next = self.token_list.next()?;
        if next.t_type != TokenType::Semicolon {
            panic!("Expect `;` after loop condition.");
        }

        let peek = self.token_list.peek();
        let increment = match peek {
            None => None,
            Some(token) => match token.t_type {
                TokenType::RightParen => None,
                _ => self.expression(),
            },
        };

        let next = self.token_list.next()?;
        if next.t_type != TokenType::RightParen {
            panic!("Expect `)` after `for` clauses.");
        }

        let mut written_body = self.next_stmt();
        if increment != None {
            let incr = Stmt::Expr(increment?);
            let stmt_vec = vec![written_body?, incr];
            written_body = Some(Stmt::Block(Block { stmt_vec }));
        }

        let condition = if written_condition == None {
            Expr::Literal(Literal {
                token: Token {
                    t_type: TokenType::True,
                    lexeme: "true".into(),
                    literal: token::Literal::Boolean(true),
                    line: next.line,
                },
            })
        } else {
            written_condition?
        };

        let body = Box::new(written_body?);

        let inner_for = Stmt::While(While { condition, body });

        let desugared_for = if initializer != None {
            Stmt::Block(Block {
                stmt_vec: vec![initializer?, inner_for],
            })
        } else {
            inner_for
        };

        Some(desugared_for)
    }
    fn if_statement(&mut self) -> Option<Stmt> {
        let next_token = self.token_list.peek();
        if next_token?.t_type != TokenType::LeftParen {
            println!("Expect ( after if")
        }
        self.token_list.next();

        let condition = self.expression()?;
        let next_token = self.token_list.peek();
        if next_token?.t_type != TokenType::RightParen {
            println!("Expect ) after if condition")
        }
        self.token_list.next();

        let if_stmt = self.next_stmt()?;
        let mut else_stmt = None;
        let next_token = self.token_list.peek();
        if next_token?.t_type == TokenType::Else {
            self.token_list.next();
            else_stmt = self.next_stmt();
        }
        Some(Stmt::If(IfStmt {
            condition,
            truth_branch: Box::new(if_stmt),
            false_branch: Box::new(else_stmt),
        }))
    }

    fn while_statement(&mut self) -> Option<Stmt> {
        let next_token = self.token_list.peek();
        if next_token?.t_type != TokenType::LeftParen {
            println!("Expect ( after while")
        }
        self.token_list.next();

        let condition = self.expression()?;
        let next_token = self.token_list.peek();
        if next_token?.t_type != TokenType::RightParen {
            println!("Expect ) after while condition")
        }
        self.token_list.next();

        if self.token_list.peek()?.t_type != TokenType::LeftBrace {
            println!("Expected a block after while condition");
            return None;
        }

        let next_stmt = self.next_stmt()?;

        let body = Box::new(next_stmt);
        let result = Stmt::While(While { condition, body });
        Some(result)
    }

    fn stmt_expr(&mut self) -> Option<Stmt> {
        let expr = self.assignment();
        let next_token = self.token_list.peek()?;

        if next_token.t_type == TokenType::EOF {
            return None;
        }

        if next_token.t_type != TokenType::Semicolon {
            println!("Expect ; after expression")
        }
        self.token_list.next();
        if expr == None {
            None
        } else {
            Some(Stmt::Expr(expr.unwrap()))
        }
    }

    fn expression(&mut self) -> Option<Expr> {
        self.assignment()
    }

    fn assignment(&mut self) -> Option<Expr> {
        let possible_expr = self.logic_or();

        if self.token_list.peek().is_none() {
            return possible_expr;
        }

        let next_token = self.token_list.peek().unwrap();

        if let TokenType::Equal = next_token.t_type {
            self.token_list.next();

            if possible_expr == None {
                panic!("Invalid assignment");
            }

            let value = self.assignment();
            if value == None {
                panic!("Invalid value on the right hand side");
            }

            let value = Box::new(value.unwrap());
            let expr = possible_expr.unwrap();

            if let Expr::Var(var) = expr {
                let name = var.name;
                let assignment = Expr::Assignment(Assignment { name, value });
                return Some(assignment);
            }

            panic!("Invalid assignment")
        } else {
            possible_expr
        }
    }

    fn logic_or(&mut self) -> Option<Expr> {
        let mut expr = self.logic_and();

        while let TokenType::Or = self.token_list.peek()?.t_type {
            let operator = self.token_list.next()?.clone();
            let left = Box::new(expr?);
            let right = Box::new(self.logic_and()?);

            expr = Some(Expr::Logical(Logical {
                left,
                right,
                operator,
            }));
        }

        expr
    }
    fn logic_and(&mut self) -> Option<Expr> {
        let mut expr = self.equality();

        while let TokenType::And = self.token_list.peek()?.t_type {
            let operator = self.token_list.next()?.clone();
            let left = Box::new(expr?);
            let right = Box::new(self.equality()?);

            expr = Some(Expr::Logical(Logical {
                left,
                right,
                operator,
            }));
        }

        expr
    }

    fn equality(&mut self) -> Option<Expr> {
        let mut expr = self.comparison();

        if self.token_list.peek() == None {
            return expr;
        }

        if let TokenType::BangEqual | TokenType::EqualEqual = self.token_list.peek()?.t_type {
            let operator = self.token_list.next()?.clone();
            let right = Box::new(self.comparison()?);
            let left = Box::new(expr?);
            expr = Some(Expr::Binary(Binary {
                left,
                right,
                operator,
            }));
        }

        expr
    }

    fn comparison(&mut self) -> Option<Expr> {
        let mut expr = self.addition();
        if self.token_list.peek() == None {
            return expr;
        }

        if let TokenType::Greater
        | TokenType::GreaterEqual
        | TokenType::Less
        | TokenType::LessEqual = self.token_list.peek()?.t_type
        {
            let operator = self.token_list.next()?.clone();
            let right = Box::new(self.multiplication()?);
            let left = Box::new(expr?);
            expr = Some(Expr::Binary(Binary {
                left,
                right,
                operator,
            }));
        }

        expr
    }

    fn addition(&mut self) -> Option<Expr> {
        let mut expr = self.multiplication();

        if self.token_list.peek() == None {
            return expr;
        }
        while let TokenType::Minus | TokenType::Plus = self.token_list.peek()?.t_type {
            let operator = self.token_list.next()?.clone();
            let right = Box::new(self.multiplication()?);
            let left = Box::new(expr?);
            expr = Some(Expr::Binary(Binary {
                left,
                right,
                operator,
            }));
            if self.token_list.peek() == None {
                break;
            }
        }

        expr
    }

    fn multiplication(&mut self) -> Option<Expr> {
        let mut expr = self.unary()?;

        if self.token_list.peek() == None {
            return Some(expr);
        }
        while let TokenType::Slash | TokenType::Star = self.token_list.peek()?.t_type {
            let operator = self.token_list.next()?.clone();
            let right = Box::new(self.unary()?);
            let left = Box::new(expr);
            expr = Expr::Binary(Binary {
                left,
                right,
                operator,
            });
            if self.token_list.peek() == None {
                break;
            }
        }

        Some(expr)
    }

    fn unary(&mut self) -> Option<Expr> {
        let peek = self.token_list.peek()?;

        if let TokenType::Bang | TokenType::Minus = peek.t_type {
            let operator = self.token_list.next()?.clone();
            let expr = Box::new(self.unary()?);
            Some(Expr::Unary(Unary { expr, operator }))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Option<Expr> {
        let peek = self.token_list.peek()?;

        match peek.t_type {
            TokenType::Identifier => {
                let name = self.token_list.next()?.clone();
                Some(Expr::Var(Var { name }))
            }
            TokenType::Number
            | TokenType::String
            | TokenType::False
            | TokenType::True
            | TokenType::Nil => {
                let token = self.token_list.next()?.clone();
                Some(Expr::Literal(Literal { token }))
            }
            TokenType::LeftParen => {
                self.token_list.next();
                let expr = self.expression()?;
                if let TokenType::RightParen = self.token_list.next().unwrap().t_type {
                    return Some(Expr::Grouping(Grouping {
                        expr: Box::new(expr),
                    }));
                }
                self.error = true;
                println!("Expecting ')' after '(' and expression");
                None
            }
            TokenType::EOF | TokenType::Semicolon => None,
            _ => {
                self.error = true;
                println!("Token not supported on primary - Expecting an expression");
                println!("{:?}", peek.line);
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lox::expr::Literal as ExprLiteral;
    use crate::lox::token::Literal;
    use pretty_assertions::assert_eq;

    #[test]
    fn literal_string() {
        let string_token = Token::new(
            TokenType::String,
            "string".to_owned(),
            Literal::String("string".into()),
            1,
        );

        let semicolon = Token::new(TokenType::Semicolon, ";".to_owned(), Literal::None, 1);

        let expected_expr = Expr::Literal(ExprLiteral {
            token: string_token.clone(),
        });

        let tokens = vec![string_token, semicolon];

        let mut parser = Parser::new(&tokens);
        let mut stmt = parser.parse();
        if let Some(Stmt::Expr(expr)) = stmt.pop() {
            assert_eq!(expr, expected_expr);
        } else {
            unreachable!()
        }
    }

    #[test]
    fn negation() {
        let number = Token::new(TokenType::Number, "1".to_owned(), Literal::F64(1.0), 1);
        let operator = Token::new(TokenType::Bang, "!".to_owned(), Literal::None, 1);

        let left = Expr::Literal(super::Literal {
            token: number.clone(),
        });
        let expected_expr = Expr::Unary(Unary {
            expr: Box::new(left),
            operator: operator.clone(),
        });

        let semicolon = Token::new(TokenType::Semicolon, ";".to_owned(), Literal::None, 1);
        let tokens = vec![operator, number, semicolon];

        let mut parser = Parser::new(&tokens);
        let mut stmt = parser.parse();
        if let Some(Stmt::Expr(expr)) = stmt.pop() {
            assert_eq!(expr, expected_expr);
        } else {
            unreachable!()
        }
    }

    #[test]
    fn addition() {
        let first_number = Token::new(TokenType::Number, "1".to_owned(), Literal::F64(1.0), 1);
        let second_number = Token::new(TokenType::Number, "2".to_owned(), Literal::F64(2.0), 1);
        let operator = Token::new(TokenType::Plus, "+".to_owned(), Literal::None, 1);

        let left = Expr::Literal(super::Literal {
            token: first_number.clone(),
        });
        let right = Expr::Literal(super::Literal {
            token: second_number.clone(),
        });
        let expected_expr = Expr::Binary(Binary {
            left: Box::new(left),
            right: Box::new(right),
            operator: operator.clone(),
        });

        let semicolon = Token::new(TokenType::Semicolon, ";".to_owned(), Literal::None, 1);
        let tokens = vec![first_number, operator, second_number, semicolon];

        let mut parser = Parser::new(&tokens);
        let mut stmt = parser.parse();
        if let Some(Stmt::Expr(expr)) = stmt.pop() {
            assert_eq!(expr, expected_expr);
        } else {
            unreachable!()
        }
    }

    #[test]
    fn equality() {
        let first_number = Token::new(TokenType::Number, "1".to_owned(), Literal::F64(1.0), 1);
        let second_number = Token::new(TokenType::Number, "1".to_owned(), Literal::F64(1.0), 1);
        let operator = Token::new(TokenType::EqualEqual, "==".to_owned(), Literal::None, 1);

        let left = Expr::Literal(super::Literal {
            token: first_number.clone(),
        });
        let right = Expr::Literal(super::Literal {
            token: second_number.clone(),
        });
        let expected_expr = Expr::Binary(Binary {
            left: Box::new(left),
            right: Box::new(right),
            operator: operator.clone(),
        });

        let semicolon = Token::new(TokenType::Semicolon, ";".to_owned(), Literal::None, 1);
        let tokens = vec![first_number, operator, second_number, semicolon];

        let mut parser = Parser::new(&tokens);
        let mut stmt = parser.parse();
        if let Some(Stmt::Expr(expr)) = stmt.pop() {
            assert_eq!(expr, expected_expr);
        } else {
            unreachable!()
        }
    }

    #[test]
    fn comparison() {
        let first_number = Token::new(TokenType::Number, "1".to_owned(), Literal::F64(1.0), 1);
        let second_number = Token::new(TokenType::Number, "2".to_owned(), Literal::F64(2.0), 1);
        let operator = Token::new(TokenType::Greater, ">".to_owned(), Literal::None, 1);
        let semicolon = Token::new(TokenType::Semicolon, ";".to_owned(), Literal::None, 1);

        let left = Expr::Literal(super::Literal {
            token: first_number.clone(),
        });
        let right = Expr::Literal(super::Literal {
            token: second_number.clone(),
        });
        let expected_expr = Expr::Binary(Binary {
            left: Box::new(left),
            right: Box::new(right),
            operator: operator.clone(),
        });

        let tokens = vec![first_number, operator, second_number, semicolon];

        let mut parser = Parser::new(&tokens);
        let mut stmt = parser.parse();
        if let Some(Stmt::Expr(expr)) = stmt.pop() {
            assert_eq!(expr, expected_expr);
        } else {
            unreachable!()
        }
    }

    #[test]
    fn multiplication() {
        let first_number = Token::new(TokenType::Number, "1".to_owned(), Literal::F64(1.0), 1);
        let operator = Token::new(TokenType::Star, "*".to_owned(), Literal::None, 1);
        let second_number = Token::new(TokenType::Number, "1".to_owned(), Literal::F64(1.0), 1);
        let semicolon = Token::new(TokenType::Semicolon, ";".to_owned(), Literal::None, 1);

        let left = Expr::Literal(super::Literal {
            token: first_number.clone(),
        });
        let right = Expr::Literal(super::Literal {
            token: second_number.clone(),
        });
        let expected_expr = Expr::Binary(Binary {
            left: Box::new(left),
            right: Box::new(right),
            operator: operator.clone(),
        });

        let tokens = vec![first_number, operator, second_number, semicolon];

        let mut parser = Parser::new(&tokens);
        let mut stmt = parser.parse();
        if let Some(Stmt::Expr(expr)) = stmt.pop() {
            assert_eq!(expr, expected_expr);
        } else {
            unreachable!()
        }
    }

    #[test]
    fn for_loop_parse() {
        // for(var a = 1; a < 2; a = a + 1) {}
        // should parse as:
        // {
        //  var a = 1;
        //  while(a < 2) {
        //   a = a + 1;
        //  }
        // }
        let for_kw = Token::new(TokenType::For, "for".to_owned(), Literal::None, 1);
        let left_paren = Token::new(TokenType::LeftParen, "(".to_owned(), Literal::None, 1);
        let var_kw = Token::new(TokenType::Var, "var".to_owned(), Literal::None, 1);
        let variable = Token::new(TokenType::Identifier, "a".to_owned(), Literal::None, 1);
        let equal_sign = Token::new(TokenType::Equal, "=".to_owned(), Literal::None, 1);
        let one = Token::new(TokenType::Number, "1".to_owned(), Literal::F64(1.0), 1);
        let semicolon = Token::new(TokenType::Semicolon, ";".to_owned(), Literal::None, 1);

        let identifier = Token::new(TokenType::Identifier, "a".to_owned(), Literal::None, 1);
        let less = Token::new(TokenType::Less, "<".to_owned(), Literal::None, 1);
        let two = Token::new(TokenType::Number, "2".to_owned(), Literal::F64(2.0), 1);
        let semicolon_two = Token::new(TokenType::Semicolon, ";".to_owned(), Literal::None, 1);

        let identifier_two = Token::new(TokenType::Identifier, "a".to_owned(), Literal::None, 1);
        let equal_sign_two = Token::new(TokenType::Equal, "=".to_owned(), Literal::None, 1);
        let identifier_three = Token::new(TokenType::Identifier, "a".to_owned(), Literal::None, 1);
        let plus_sign = Token::new(TokenType::Plus, "+".to_owned(), Literal::None, 1);
        let one_again = Token::new(TokenType::Number, "1".to_owned(), Literal::F64(1.0), 1);
        let right_paren = Token::new(TokenType::RightParen, ")".to_owned(), Literal::None, 1);
        let left_bracket = Token::new(TokenType::LeftBrace, "{".to_owned(), Literal::None, 1);
        let right_bracket = Token::new(TokenType::RightBrace, "}".to_owned(), Literal::None, 1);

        let tokens = vec![
            for_kw,
            left_paren,
            var_kw,
            variable.clone(),
            equal_sign,
            one.clone(),
            semicolon,
            identifier,
            less.clone(),
            two.clone(),
            semicolon_two,
            identifier_two,
            equal_sign_two,
            identifier_three,
            plus_sign.clone(),
            one_again,
            right_paren,
            left_bracket,
            right_bracket,
        ];

        let declaration = Stmt::Declaration(stmt::Var {
            value: Expr::Literal(super::Literal { token: one.clone() }),
            name: 'a'.to_string(),
        });
        let while_left = Expr::Var(super::Var {
            name: variable.clone(),
        });
        let while_right = Expr::Literal(super::Literal { token: two.clone() });
        let while_greater_expr = Expr::Binary(Binary {
            left: Box::new(while_left),
            right: Box::new(while_right),
            operator: less.clone(),
        });

        let block_var = Expr::Var(super::Var {
            name: variable.clone(),
        });
        let block_value = Expr::Literal(super::Literal { token: one.clone() });
        let block_right = Expr::Binary(Binary {
            left: Box::new(block_var),
            right: Box::new(block_value),
            operator: plus_sign.clone(),
        });
        let block_left = Stmt::Expr(Expr::Assignment(super::Assignment {
            name: variable,
            value: Box::new(block_right),
        }));

        let inner_block_for = Stmt::Block(super::Block { stmt_vec: vec![] });
        let block = Stmt::Block(super::Block {
            stmt_vec: vec![inner_block_for, block_left],
        });

        let while_stmt = Stmt::While(While {
            condition: while_greater_expr,
            body: Box::new(block),
        });

        let desugared_for = Stmt::Block(super::Block {
            stmt_vec: vec![declaration, while_stmt],
        });

        let mut parser = Parser::new(&tokens);
        let mut stmt = parser.parse();
        assert_eq!(desugared_for, stmt.pop().unwrap());
    }
    #[test]
    fn while_statement_parse() {
        // while(a < 2) {
        //   a = a + 1;
        // }
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
            plus_sign.clone(),
            one.clone(),
            semicolon,
            right_bracket,
        ];

        let while_left = Expr::Var(super::Var {
            name: variable.clone(),
        });
        let while_right = Expr::Literal(super::Literal { token: two.clone() });
        let while_greater_expr = Expr::Binary(Binary {
            left: Box::new(while_left),
            right: Box::new(while_right),
            operator: greater.clone(),
        });

        let block_var = Expr::Var(super::Var {
            name: variable.clone(),
        });
        let block_value = Expr::Literal(super::Literal { token: one.clone() });
        let block_right = Expr::Binary(Binary {
            left: Box::new(block_var),
            right: Box::new(block_value),
            operator: plus_sign.clone(),
        });
        let block_left = Stmt::Expr(Expr::Assignment(super::Assignment {
            name: variable,
            value: Box::new(block_right),
        }));

        let block = Stmt::Block(super::Block {
            stmt_vec: vec![block_left],
        });

        let while_stmt = Stmt::While(While {
            condition: while_greater_expr,
            body: Box::new(block),
        });

        let mut parser = Parser::new(&tokens);
        let mut stmt = parser.parse();
        assert_eq!(while_stmt, stmt.pop().unwrap());
    }
}
