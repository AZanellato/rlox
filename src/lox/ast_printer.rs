use super::expr::{Binary, Expr, Grouping, Unary};
use std::io::{self, Write};

pub fn print_node(expr: Expr) {
    match expr {
        Expr::Literal(expr) => println!("({})", expr.token.lexeme),
        Expr::Grouping(expr) => print_grouping(expr),
        Expr::Binary(expr) => print_binary(expr),
        Expr::Unary(expr) => print_unary(expr),
    }

    io::stdout().flush().unwrap();
}

fn print_binary(expr: Binary) {
    print!("({} ", expr.operator.lexeme);
    print_node(*expr.left);
    print_node(*expr.right);
    print!(")");
}
fn print_unary(expr: Unary) {
    print!("({} ", expr.operator);
    print_node(*expr.expr);
    print!(")");
}

fn print_grouping(expr: Grouping) {
    print!("group (");
    print_node(*expr.expr);
    print!(")");
}
