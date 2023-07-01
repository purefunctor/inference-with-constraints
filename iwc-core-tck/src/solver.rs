pub mod solve;

use std::collections::HashMap;

use iwc_core_ast::ty::{Assertion, TypeIdx};

use crate::context::Context;

pub struct Solver {
    pub(crate) context: Context,
    pub(crate) unifications: HashMap<usize, TypeIdx>,
    pub(crate) unsolved_deep: Vec<(usize, usize)>,
}

impl Solver {
    pub fn new(context: Context) -> Self {
        Self {
            context,
            unifications: HashMap::new(),
            unsolved_deep: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub enum Constraint {
    ClassAssertion(Assertion),
    UnifyDeep(usize, usize),
    UnifySolve(usize, TypeIdx),
}
