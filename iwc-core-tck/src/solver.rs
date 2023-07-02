pub mod solve;

use std::collections::HashMap;

use iwc_core_ast::ty::TypeIdx;
use iwc_core_constraints::UnifyError;

use crate::context::Context;

pub struct Solver {
    pub(crate) context: Context,
    pub(crate) unification_solved: HashMap<usize, TypeIdx>,
    pub(crate) unification_unsolved: Vec<(usize, usize)>,
    pub(crate) unification_errors: Vec<UnifyError>,
}

impl Solver {
    pub fn new(context: Context) -> Self {
        Self {
            context,
            unification_solved: HashMap::new(),
            unification_unsolved: Vec::new(),
            unification_errors: Vec::new(),
        }
    }
}
