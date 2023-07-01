pub mod solve;

use std::collections::HashMap;

use iwc_core_ast::ty::{Assertion, TypeIdx};

use crate::context::Context;

pub struct Solver {
    pub(crate) context: Context,
    pub(crate) unification_solved: HashMap<usize, TypeIdx>,
    pub(crate) unification_unsolved: Vec<(usize, usize)>,
}

impl Solver {
    pub fn new(context: Context) -> Self {
        Self {
            context,
            unification_solved: HashMap::new(),
            unification_unsolved: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub enum Constraint {
    ClassAssertion(Assertion),
    UnifyDeep(usize, usize),
    UnifySolve(usize, TypeIdx),
}
