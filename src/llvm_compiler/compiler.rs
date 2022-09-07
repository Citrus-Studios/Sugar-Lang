use crate::parser::{Expr, Expr_};

pub struct Compiler {
    ast: Vec<Expr>,
}

impl Compiler {
    pub fn new(ast: Vec<Expr>) -> Self {
        Self { ast }
    }

    pub fn compile_llvm(self) {
        for outer in self.ast {
            match outer.node {
                Expr_::Declare(name, types) => {}
                _ => panic!("Unexpected Expression"),
            }
        }
    }
}
