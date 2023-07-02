use iwc_core_ast::ty::{Assertion, TypeIdx};
use iwc_core_error::UnifyError;

#[derive(Debug)]
pub enum Constraint {
    ClassAssertion(Assertion),
    UnifyDeep(usize, usize),
    UnifySolve(usize, TypeIdx),
    UnifyError(UnifyError),
}
