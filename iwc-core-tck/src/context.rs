pub mod environment;
pub mod infer;
pub mod instantiate;
pub mod unify;
pub mod volatile;

use std::collections::HashMap;

use iwc_arena::Arena;
use iwc_core_ast::{
    expr::Expr,
    ty::{Type, TypeIdx},
};
use iwc_core_constraints::Constraint;
use smol_str::SmolStr;

use crate::solver::Solver;

#[derive(Default)]
pub struct Environment {
    pub(crate) constructor_bindings: HashMap<SmolStr, TypeIdx>,
    pub(crate) value_bindings: HashMap<SmolStr, TypeIdx>,
}

#[derive(Default)]
pub struct Volatile {
    pub(crate) expr_arena: Arena<Expr>,
    pub(crate) type_arena: Arena<Type>,
    pub(crate) fresh_index: usize,
    pub(crate) constraints: Vec<Constraint>,
}

#[derive(Default)]
pub struct Context {
    pub(crate) environment: Environment,
    pub(crate) volatile: Volatile,
}

impl Context {
    pub fn solver(self) -> Solver {
        Solver::new(self)
    }
}
