pub mod ast_printer;
pub mod expr;
pub mod interpreter;
pub mod parser;
pub mod scanner;
pub mod stmt;
pub mod token;
use std::fs;
use std::process;
extern crate derive_more;
extern crate phf;
extern crate rustyline;

use interpreter::Interpreter;
use rustyline::error::ReadlineError;
use rustyline::Editor;

#[derive(Debug)]
pub struct Lox {
    had_errors: bool,
    had_runtime_errors: bool,
}

impl Lox {
    pub fn new() -> Lox {
        Lox {
            had_errors: false,
            had_runtime_errors: false,
        }
    }

    pub fn prompt(&mut self) -> () {
        let mut interpreter = Interpreter::new();
        let mut rl = Editor::<()>::new();
        loop {
            let readline = rl.readline(">> ");
            match readline {
                Ok(line) => {
                    rl.add_history_entry(line.as_str());
                    self.run(&mut interpreter, line);
                    self.had_errors = false;
                }
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    break;
                }
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    break;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }
    }

    pub fn runfile(&self, path: std::path::PathBuf) -> () {
        let source = fs::read_to_string(path).unwrap_or("".to_string());
        let mut interpreter = Interpreter::new();
        self.run(&mut interpreter, source);
        if self.had_errors {
            process::exit(1);
        }
    }

    fn run(&self, interpreter: &mut Interpreter, source: String) {
        let mut scanner = scanner::Scanner::new(&source);
        let tokens = scanner.scan_tokens();
        let mut parser = parser::Parser::new(tokens);
        let statements = parser.parse();
        for node in statements {
            interpreter.evaluate_node(node);
        }
    }

    // fn error(&mut self, line: u32, message: String) -> () {
    //     self.report(line, "".to_string(), message);
    // }

    // fn report(&mut self, line_number: u32, locale: String, error_msg: String) -> () {
    //     println!("line: {} Error {}: {}", line_number, locale, error_msg);
    //     self.had_errors = true;
    // }
}
