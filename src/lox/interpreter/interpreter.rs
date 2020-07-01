use super::environment::Environment;
use crate::lox::expr::Var as Var_expr;
use crate::lox::expr::{Assignment, Binary, Expr, Literal, Logical, Unary};
use crate::lox::stmt::{Block, IfStmt, Stmt, Var, While};
use crate::lox::token;
use derive_more::Display;
use std::cell::RefCell;
use std::ops::{Add, Div, Mul, Neg, Not, Sub};
use std::rc::Rc;

#[derive(PartialEq, PartialOrd, Debug, Display, Clone)]
pub enum Value {
    String(String),
    F64(f64),
    Boolean(bool),
    Nil,
}

#[derive(Debug)]
pub struct Interpreter {
    env: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let env = Rc::new(RefCell::new(Environment::new()));
        Self { env }
    }

    pub fn evaluate_node(&mut self, stmt: Stmt) -> Value {
        match stmt {
            Stmt::Expr(expr) => self.evaluate_expression(expr),
            Stmt::Print(expr) => self.evaluate_print(expr),
            Stmt::Declaration(var) => self.evaluate_declaration(var),
            Stmt::Block(block) => self.evaluate_block(block),
            Stmt::If(block) => self.evaluate_if(block),
            Stmt::While(block) => self.evaluate_while(block),
        }
    }

    fn evaluate_expression(&mut self, expr: Expr) -> Value {
        match expr {
            Expr::Literal(expr) => self.evaluate_literal(expr),
            Expr::Logical(expr) => self.evaluate_logical(expr),
            Expr::Unary(expr) => self.evaluate_unary(expr),
            Expr::Binary(expr) => self.evaluate_binary(expr),
            Expr::Var(expr) => self.evaluate_variable(expr),
            Expr::Assignment(expr) => self.evaluate_assignment(expr),
            Expr::Grouping(_) => panic!("Grouping not implemented"),
        }
    }

    fn evaluate_print(&mut self, expr: Expr) -> Value {
        let value = self.evaluate_expression(expr);
        println!("{}", value);
        Value::Nil
    }

    fn evaluate_logical(&mut self, expr: Logical) -> Value {
        let left = self.evaluate_expression(*expr.left);

        match expr.operator.t_type {
            token::TokenType::Or => {
                if left.truthyness() {
                    return left;
                }
                self.evaluate_expression(*expr.right)
            }
            token::TokenType::And => {
                if !left.truthyness() {
                    return left;
                }

                self.evaluate_expression(*expr.right)
            }
            _ => panic!("Not a logical operator"),
        }
    }

    fn evaluate_declaration(&mut self, var: Var) -> Value {
        let value = self.evaluate_expression(var.value);
        self.env.borrow_mut().define(&var.name, value);
        self.env.borrow_mut().get(&var.name).unwrap().clone()
    }

    fn evaluate_block(&mut self, block: Block) -> Value {
        let prev_env = Rc::clone(&self.env);
        let env = Rc::new(RefCell::new(Environment::new()));
        self.env = env;
        self.env.borrow_mut().enclose(prev_env.clone());
        for stmt in block.stmt_vec {
            self.evaluate_node(stmt);
        }
        self.env = prev_env;
        Value::Nil
    }
    fn evaluate_assignment(&mut self, assignment_expr: Assignment) -> Value {
        let expr = assignment_expr.value;
        let value = self.evaluate_expression(*expr);
        let name = assignment_expr.name.lexeme;
        self.env.borrow_mut().assign(&name, value);
        let value = self.env.borrow_mut().get(&name);
        value.unwrap().clone()
    }

    fn evaluate_if(&mut self, if_statement: IfStmt) -> Value {
        let truth_branch = *if_statement.truth_branch;
        let false_branch = *if_statement.false_branch;
        let condition = self.evaluate_expression(if_statement.condition);

        if condition.truthyness() {
            self.evaluate_node(truth_branch)
        } else if false_branch != None {
            self.evaluate_node(false_branch.unwrap())
        } else {
            Value::Nil
        }
    }

    fn evaluate_while(&mut self, while_stmt: While) -> Value {
        let mut condition = self.evaluate_expression(while_stmt.condition.clone());
        while condition.truthyness() {
            let body = *while_stmt.body.clone();
            self.evaluate_node(body);
            condition = self.evaluate_expression(while_stmt.condition.clone());
        }

        Value::Nil
    }

    fn evaluate_variable(&mut self, expr: Var_expr) -> Value {
        let identifier = expr.name;
        let name = identifier.lexeme;
        match self.env.borrow_mut().get(&name) {
            Some(value) => value.clone(),
            None => panic!("The variable {var_name} doesn't exist", var_name = name),
        }
    }

    fn evaluate_literal(&mut self, expr: Literal) -> Value {
        match expr.token.literal {
            token::Literal::String(string) => Value::String(string),
            token::Literal::F64(f64) => Value::F64(f64),
            token::Literal::Boolean(boolean) => Value::Boolean(boolean),
            _ => Value::Nil,
        }
    }

    fn evaluate_unary(&mut self, unary_expr: Unary) -> Value {
        let value = self.evaluate_expression(*unary_expr.expr);

        match unary_expr.operator.t_type {
            token::TokenType::Minus => -value,
            token::TokenType::Bang => !value,
            _ => Value::Nil,
        }
    }

    fn evaluate_binary(&mut self, expr: Binary) -> Value {
        let left_value = self.evaluate_expression(*expr.left);
        let right_value = self.evaluate_expression(*expr.right);

        match expr.operator.t_type {
            token::TokenType::Plus => left_value + right_value,
            token::TokenType::Minus => left_value - right_value,
            token::TokenType::Slash => left_value / right_value,
            token::TokenType::Star => left_value * right_value,
            token::TokenType::Greater => Value::Boolean(left_value > right_value),
            token::TokenType::GreaterEqual => Value::Boolean(left_value >= right_value),
            token::TokenType::Less => Value::Boolean(left_value < right_value),
            token::TokenType::LessEqual => Value::Boolean(left_value <= right_value),
            token::TokenType::EqualEqual => Value::Boolean(left_value == right_value),
            token::TokenType::BangEqual => Value::Boolean(left_value != right_value),
            _ => panic!("Not implemented"),
        }
    }
}
impl Value {
    pub fn truthyness(&self) -> bool {
        match *self {
            Value::Boolean(boolean) => boolean,
            Value::Nil => false,
            _ => true,
        }
    }
}

impl Add for Value {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Value::F64(left), Value::F64(right)) => Value::F64(left + right),
            (Value::String(left), Value::String(right)) => {
                let mut new_string = left;
                new_string.push_str(&right);
                Value::String(new_string)
            }
            (_, _) => panic!("Not implemented"),
        }
    }
}

impl Sub for Value {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Value::F64(left), Value::F64(right)) => Value::F64(left - right),
            (_, _) => panic!("Not implemented"),
        }
    }
}

impl Div for Value {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Value::F64(left), Value::F64(right)) => Value::F64(left / right),
            (_, _) => panic!("Not implemented"),
        }
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Value::F64(left), Value::F64(right)) => Value::F64(left * right),
            (_, _) => panic!("Not implemented"),
        }
    }
}

impl Neg for Value {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            Value::F64(value) => Value::F64(-value),
            _ => panic!("Expected a number, got: {:?}", self),
        }
    }
}

impl Not for Value {
    type Output = Self;

    fn not(self) -> Self {
        Value::Boolean(!self.truthyness())
    }
}

mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::lox::expr::Literal as ExprLiteral;
    use crate::lox::token::{Literal, Token, TokenType};

    #[test]
    fn literal_string() {
        let mut interpreter = Interpreter::new();
        let expr = Expr::Literal(ExprLiteral {
            token: Token::new(
                TokenType::String,
                "string".to_owned(),
                Literal::String("string".into()),
                1,
            ),
        });

        let value = interpreter.evaluate_expression(expr);
        assert_eq!(value, Value::String("string".into()));
    }

    #[test]
    fn negation() {
        let mut interpreter = Interpreter::new();
        let operator = Token::new(TokenType::Bang, "!".to_owned(), Literal::None, 1);

        let left = Expr::Literal(super::Literal {
            token: Token::new(TokenType::Number, "1".to_owned(), Literal::F64(1.0), 1),
        });
        let expr = Expr::Unary(Unary {
            expr: Box::new(left),
            operator,
        });

        let value = interpreter.evaluate_expression(expr);
        assert_eq!(value, Value::Boolean(false));
    }

    #[test]
    fn addition() {
        let mut interpreter = Interpreter::new();
        let operator = Token::new(TokenType::Plus, "+".to_owned(), Literal::None, 1);

        let left = Expr::Literal(super::Literal {
            token: Token::new(TokenType::Number, "1".to_owned(), Literal::F64(1.0), 1),
        });
        let right = Expr::Literal(super::Literal {
            token: Token::new(TokenType::Number, "2".to_owned(), Literal::F64(2.0), 1),
        });
        let expr = Expr::Binary(Binary {
            left: Box::new(left),
            right: Box::new(right),
            operator,
        });

        let value = interpreter.evaluate_expression(expr);
        assert_eq!(value, Value::F64(3.0));
    }

    #[test]
    fn equality() {
        let mut interpreter = Interpreter::new();
        let operator = Token::new(TokenType::EqualEqual, "==".to_owned(), Literal::None, 1);

        let left = Expr::Literal(super::Literal {
            token: Token::new(TokenType::Number, "1".to_owned(), Literal::F64(1.0), 1),
        });
        let right = Expr::Literal(super::Literal {
            token: Token::new(TokenType::Number, "1".to_owned(), Literal::F64(1.0), 1),
        });
        let expr = Expr::Binary(Binary {
            left: Box::new(left),
            right: Box::new(right),
            operator,
        });

        let value = interpreter.evaluate_expression(expr);
        assert_eq!(value, Value::Boolean(true))
    }

    #[test]
    fn comparison() {
        let mut interpreter = Interpreter::new();
        let operator = Token::new(TokenType::Greater, ">".to_owned(), Literal::None, 1);

        let left = Expr::Literal(super::Literal {
            token: Token::new(TokenType::Number, "1".to_owned(), Literal::F64(1.0), 1),
        });
        let right = Expr::Literal(super::Literal {
            token: Token::new(TokenType::Number, "2".to_owned(), Literal::F64(2.0), 1),
        });
        let expr = Expr::Binary(Binary {
            left: Box::new(left),
            right: Box::new(right),
            operator,
        });

        let value = interpreter.evaluate_expression(expr);
        assert_eq!(value, Value::Boolean(false))
    }

    #[test]
    fn multiplication() {
        let mut interpreter = Interpreter::new();
        let operator = Token::new(TokenType::Star, "*".to_owned(), Literal::None, 1);

        let left = Expr::Literal(super::Literal {
            token: Token::new(TokenType::Number, "3".to_owned(), Literal::F64(3.0), 1),
        });
        let right = Expr::Literal(super::Literal {
            token: Token::new(TokenType::Number, "3".to_owned(), Literal::F64(3.0), 1),
        });
        let expr = Expr::Binary(Binary {
            left: Box::new(left),
            right: Box::new(right),
            operator,
        });

        let value = interpreter.evaluate_expression(expr);
        assert_eq!(value, Value::F64(9.0));
    }

    #[test]
    fn while_loop_interpreter() {
        let variable = Token::new(TokenType::Identifier, "a".to_owned(), Literal::None, 1);
        let greater = Token::new(TokenType::Less, "<".to_owned(), Literal::None, 1);
        let two = Token::new(TokenType::Number, "2".to_owned(), Literal::F64(2.0), 1);
        let plus_sign = Token::new(TokenType::Plus, "+".to_owned(), Literal::None, 1);
        let one = Token::new(TokenType::Number, "1".to_owned(), Literal::F64(1.0), 1);

        let var_dcl = Stmt::Declaration(Var {
            name: "a".to_owned(),
            value: Expr::Literal(crate::lox::expr::Literal { token: one.clone() }),
        });

        let while_left = Expr::Var(crate::lox::expr::Var {
            name: variable.clone(),
        });
        let while_right = Expr::Literal(crate::lox::expr::Literal { token: two.clone() });
        let while_greater_expr = Expr::Binary(Binary {
            left: Box::new(while_left),
            right: Box::new(while_right),
            operator: greater.clone(),
        });

        let block_var = Expr::Var(crate::lox::expr::Var {
            name: variable.clone(),
        });
        let block_value = Expr::Literal(crate::lox::expr::Literal { token: one.clone() });
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

        let mut interpreter = Interpreter::new();

        interpreter.evaluate_node(var_dcl);
        interpreter.evaluate_node(while_stmt);
        assert_eq!(
            interpreter.env.borrow_mut().get("a").unwrap(),
            Value::F64(2.0)
        );
    }
}
