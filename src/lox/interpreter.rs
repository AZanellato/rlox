use super::expr::Var as Var_expr;
use super::expr::{Assignment, Binary, Expr, Literal, Unary};
use super::stmt::{Block, Stmt, Var};
use super::token;
use derive_more::Display;
use std::collections::HashMap;
use std::ops::{Add, Div, Mul, Neg, Not, Sub};

#[derive(PartialEq, PartialOrd, Debug, Display, Clone)]
pub enum Value {
    String(String),
    F64(f64),
    Boolean(bool),
    Nil,
}

#[derive(Clone, Debug)]
struct Environment {
    env_values: HashMap<String, Value>,
    enclosed: Option<Box<Environment>>,
}

impl Environment {
    fn new() -> Self {
        Self {
            env_values: HashMap::new(),
            enclosed: None,
        }
    }

    fn enclosed(enclosed: Box<Environment>) -> Self {
        Self {
            env_values: HashMap::new(),
            enclosed: Some(enclosed),
        }
    }

    fn define(&mut self, name: &str, value: Value) {
        let new_name = name.into();
        self.env_values.insert(new_name, value);
    }

    fn assign(&mut self, name: &str, value: Value) {
        match self.get(name) {
            Some(_) => self.define(name, value),
            None => self.define_enclosed(name, value),
        }
    }

    fn define_enclosed(&mut self, name: &str, value: Value) {
        match &mut self.enclosed {
            Some(boxed_env) => boxed_env.assign(name, value),
            None => println!("Variable not declared with name: {}", name),
        }
    }

    fn get(&self, name: &str) -> Option<&Value> {
        let var_in_context = self.env_values.get(name);
        if let Some(var) = var_in_context {
            Some(var)
        } else {
            match &self.enclosed {
                None => None,
                Some(boxed_env) => boxed_env.get(name),
            }
        }
    }
}

#[derive(Debug)]
pub struct Interpreter {
    env: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        let env = Environment::new();
        Self { env }
    }

    pub fn evaluate_node(&mut self, stmt: Stmt) -> Value {
        match stmt {
            Stmt::Expr(expr) => self.evaluate_expression(expr),
            Stmt::Print(expr) => self.evaluate_print(expr),
            Stmt::Declaration(var) => self.evaluate_declaration(var),
            Stmt::Block(block) => self.evaluate_block(block),
            Stmt::If(var) => Value::Nil,
        }
    }

    fn evaluate_expression(&mut self, expr: Expr) -> Value {
        match expr {
            Expr::Literal(expr) => self.evaluate_literal(expr),
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

    fn evaluate_declaration(&mut self, var: Var) -> Value {
        let value = self.evaluate_expression(var.value);
        self.env.define(&var.name, value);
        self.env.get(&var.name).unwrap().clone()
    }

    fn evaluate_block(&mut self, block: Block) -> Value {
        let prev_env = self.env.clone();
        let new_env = Environment::enclosed(Box::new(self.env.clone()));
        self.env = new_env;
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
        self.env.assign(&name, value);
        self.env.get(&name).unwrap().clone()
    }

    fn evaluate_variable(&mut self, expr: Var_expr) -> Value {
        let identifier = expr.name;
        let name = identifier.lexeme;
        self.env.get(&name).unwrap().clone()
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
}
