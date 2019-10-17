mod lox;
use lox::Lox;
use std::{env, path};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut lox = Lox::new();
    match args.len() {
        1 => lox.prompt(),
        2 => {
            let path = path::PathBuf::from(&args[1]);
            lox.runfile(path);
        }
        _ => {
            println!("Usage rlox [script]");
            std::process::exit(1);
        }
    }
}
