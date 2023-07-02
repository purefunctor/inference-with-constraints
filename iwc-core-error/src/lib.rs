use iwc_core_ast::ty::TypeIdx;

#[derive(Debug)]
pub enum UnifyError {
    CannotUnify(TypeIdx, TypeIdx),
    ImpredicativeType(usize, TypeIdx),
    InfiniteType(usize, TypeIdx),
    InvalidArity(TypeIdx, usize, TypeIdx, usize),
}
