use std::process::Command;

use crate::Args;

pub fn add_commands(args: Args) {
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
}
