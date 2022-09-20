use inkwell::{context::Context, module::Module};

pub struct Compiler<'a> {
	ast: Vec<Expr>,
	context: Context,
	module: Option<Module<'a>>,
}

impl<'a> Compiler<'a> {
	pub fn new(ast: Vec<Expr>) -> Self {
		Self {
			ast,
			context: Context::create(),
			module: None,
		}
	}

	pub fn compile_llvm(mut self) {
		self.module = Some(self.context.create_module("main"));
		for outer in self.ast {
			match outer.node {
				Expr_::Declare(name, types) => {}
				_ => panic!("Unexpected Expression"),
			}
		}
	}
}
