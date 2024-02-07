mod core;
mod runtime;
mod syntax;

use runtime::Interpreter;
use std::env;
use std::fs;
use syntax::lex;
use syntax::parse;

use crate::runtime::ScopesStack;

pub const KEYWORDS: [&str; 5] = ["print", "let", "if", "true", "false"];

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = if args.len() > 1 {
        &args[1]
    } else {
        println!("[cei] an ego file is required");
        std::process::exit(1);
    };

    if !filename.ends_with(".e") {
        println!("[cei] This is not .e (ego) file");
        std::process::exit(1);
    }

    let file_content = fs::read_to_string(filename).expect("[cei] This is not .e (ego) file");

    let tokens = lex(file_content);
    if args.len() > 2 && args[2] == "-d" {
        println!("\nLexer tokens: \n-------------");
        for (i, token) in tokens.iter().enumerate() {
            println!("{i}. {token}");
        }
    }

    let ast = parse(tokens.clone());
    if args.len() > 2 && args[2] == "-d" {
        println!("\nAST:\n----\n{:#?}\n", ast);
    }

    Interpreter::new(ScopesStack::new(), ast).exec();
}
