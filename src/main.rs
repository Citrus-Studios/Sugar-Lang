use clap::Parser as ClapParser;
use lexer::Lexer;
use parser::Parser;
use std::fs::File;
use std::io::prelude::*;

mod ast;
mod lexer;
mod parser;
mod tokens;

#[derive(ClapParser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value = "main.sug")]
    file: String,
}

fn main() {
    let args = Args::parse();
    let contents = {
        let mut file = File::open(args.file.as_str()).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents
    };
    let tokens = Lexer::new(contents).run();
    println!("{:#?}", tokens);
    let ast = Parser::new(tokens).run();
}
