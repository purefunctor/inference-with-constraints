use iwc_core_ast::ty::{Assertion, TypeIdx};

#[derive(Debug)]
pub enum Constraint {
    ClassAssertion(Assertion),
    UnifyDeep(usize, usize),
    UnifySolve(usize, TypeIdx),
    UnifyError(UnifyError),
}

#[derive(Debug)]
pub enum UnifyError {
    CannotUnify(TypeIdx, TypeIdx),
    ImpredicativeType(usize, TypeIdx),
    InfiniteType(usize, TypeIdx),
    InvalidArity(TypeIdx, usize, TypeIdx, usize),
}
