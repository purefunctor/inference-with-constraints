pub mod solve;

use iwc_core_ast::ty::{Assertion, TypeIdx};

use crate::context::Context;

pub struct Solver {
    pub(crate) context: Context,
}

impl Solver {
    pub fn new(context: Context) -> Self {
        Self { context }
    }
}

#[derive(Debug)]
pub enum Constraint {
    ClassAssertion(Assertion),
    UnifyDeep(usize, usize),
    UnifySolve(usize, TypeIdx),
}
