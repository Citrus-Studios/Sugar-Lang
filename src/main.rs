#![feature(let_chains)]

use crate::codegen::CodeGen;
use crate::rid::RIDStruct;
use clap::Parser as ClapParser;
use lexer::Lexer;
use parser::Parser;
use std::fs::File;
use std::io::prelude::*;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod ast;
mod codegen;
mod lexer;
mod parser;
mod rid;
mod tokens;

#[derive(ClapParser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value = "main.sug")]
    file: String,
}

fn main() {
    // Subscriber Stuff
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed Setting Global Subscriber");

    let args = Args::parse();
    let contents = {
        let mut file = File::open(args.file.as_str()).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents
    };
    let tokens = Lexer::new(contents).run();
    info!("Lexer: {:#?}", tokens);
    let ast = Parser::new(tokens).run();
    info!("AST: {:#?}", ast);
    let rid = RIDStruct::new(ast).run();
    info!("RID: {:#?}", rid);
    let codegen = CodeGen::new(rid).run();
}
