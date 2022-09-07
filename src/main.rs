use clap::Parser as ClapParser;
use std::io::prelude::*;
use std::time::Instant;
use std::{fs::File, process::Command};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod lexer;
mod llvm;
mod parser;

#[derive(ClapParser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value = "main.sug")]
    file: String,
    #[clap(short, long)]
    printing: bool,
    #[clap(short, long)]
    strip: bool,
    #[clap(short, long)]
    release: bool,
    #[clap(short = 'S', long)]
    r#static: bool,
}

fn main() {
    let now = Instant::now();

    let command = if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "mkdir build"]).output()
    } else {
        Command::new("sh").args(["-c", "mkdir build"]).output()
    };
    command.unwrap();

    let args = Args::parse();
    if args.printing {
        // Subscriber Stuff
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::TRACE)
            .finish();
        tracing::subscriber::set_global_default(subscriber)
            .expect("Failed Setting Global Subscriber");
    }
    let contents = {
        let mut file = File::open(args.file.as_str()).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents
    };
    let lexer = lexer::Lexer::new(&contents).inspect(|tok| info!("tok: {:?}", tok));
    let program = parser::parse(lexer).unwrap();

    info!("{:#?}", program.stmts);

    let _ = llvm::Compiler::new(program.stmts).compile_llvm();

    let prefix;
    let cmd;

    if cfg!(target_os = "windows") {
        prefix = "/C";
        cmd = "cmd";
    } else {
        prefix = "-c";
        cmd = "sh";
    }
    Command::new(cmd)
        .args([prefix, "llvm-dis out.bc"])
        .output()
        .unwrap();

    Command::new(cmd)
        .args([prefix, "llc -filetype=obj out.ll -o out.o"])
        .output()
        .unwrap();
    if args.r#static {
        if args.release {
            Command::new(cmd)
                .args([prefix, "ld.lld -O3 -static out.o -o out;"])
                .output()
                .unwrap();
        } else {
            Command::new(cmd)
                .args([prefix, "ld.lld -static out.o -o out;"])
                .output()
                .unwrap();
        }
    } else {
        if args.release {
            Command::new(cmd)
                .args([prefix, "ld.lld -O3 out.o -o out;"])
                .output()
                .unwrap();
        } else {
            Command::new(cmd)
                .args([prefix, "ld.lld out.o -o out;"])
                .output()
                .unwrap();
        }
    }
    if args.strip {
        Command::new(cmd)
            .args([prefix, "strip out;"])
            .output()
            .unwrap();
    }

    let elapsed = now.elapsed();
    info!("Elapsed: {:.3?}", elapsed);
}
