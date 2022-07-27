use crate::ast::ASTStruct;

pub struct CodeGen {
    ast: ASTStruct,
}

impl CodeGen {
    pub fn new(ast: ASTStruct) -> Self {
        Self { ast }
    }
    pub fn run(mut self) {
        todo!();
    }
}
