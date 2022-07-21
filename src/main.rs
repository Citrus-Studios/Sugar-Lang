use clap::Parser;
use std::fs::File;
use std::io::prelude::*;

mod tokens;
mod lexer;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value="main.sug")]
    file: String
}

fn main() {
    let args = Args::parse();
    let contents = {
        let mut file = File::open(args.file.as_str()).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents
    };
}
