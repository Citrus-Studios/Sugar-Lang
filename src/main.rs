use clap::Parser as ClapParser;
use std::io::prelude::*;
use std::time::Instant;
use std::{fs::File, process::Command};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use crate::commands::add_commands;

mod commands;
// mod llvm_compiler;
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
	// let lexer = lexer::Lexer::new(&contents).inspect(|tok| info!("tok: {:?}", tok));

	// info!("{:#?}", program.stmts);

	// let _ = Compiler::new(program.stmts).compile_llvm();

	add_commands(args);

	let elapsed = now.elapsed();
	info!("Elapsed: {:.3?}", elapsed);
}
