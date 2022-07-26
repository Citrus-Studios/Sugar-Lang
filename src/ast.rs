pub enum AST<T> {
    Block { scope: usize, contents: Vec<AST<T>> },
    Literal(T),
}
