use crate::ast::{ASTStruct, Symbol};

pub enum RID<'a> {
    Block { contents: Vec<RIDData<'a>> },
    Byte(u8),
    Arg(RIDData<'a>),
    Function(String, Vec<RIDData<'a>>, RIDData<'a>),
    Var(String, RIDData<'a>),
    Symbol(Symbol),
    Comparison((RIDData<'a>, RIDData<'a>, RIDData<'a>)),
    If(RIDData<'a>),
    IfElse(RIDData<'a>),
    Else,
}

pub struct RIDData<'a> {
    rid: &'a RID<'a>,
    scope: usize,
    line: usize,
    char_pos: usize,
}

pub struct RIDStruct<'a> {
    ast: ASTStruct,
    rid: RIDData<'a>,
}

impl<'a> RIDStruct<'a> {
    pub fn new(ast: ASTStruct) -> Self {
        Self { ast }
    }
    pub fn run(mut self) {}
}
