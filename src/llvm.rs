use crate::parser::Expr;

pub struct Compiler {
    ast: Vec<Expr>,
}

impl Compiler {
    pub fn new(ast: Vec<Expr>) -> Self {
        Self { ast }
    }

    pub fn compile_llvm(self) {}
}
