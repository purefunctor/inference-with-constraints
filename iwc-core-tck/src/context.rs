pub mod common;
pub mod entail;
pub mod env;
pub mod infer;
pub mod instantiate;
pub mod solve;
pub mod unify;

use std::collections::HashMap;

use iwc_arena::Arena;
use iwc_core_ast::{
    expr::Expr,
    ty::{Assertion, Instance, Ty, TyIdx},
};
use smol_str::SmolStr;

#[derive(Default)]
pub struct Environment {
    pub(crate) bindings: HashMap<SmolStr, TyIdx>,
    pub(crate) instances: HashMap<SmolStr, Vec<Instance>>,
}

#[derive(Default)]
pub struct Volatile {
    pub(crate) expr_arena: Arena<Expr>,
    pub(crate) ty_arena: Arena<Ty>,
    pub(crate) fresh_index: usize,
    pub(crate) constraints: Vec<Constraint>,
}

#[derive(Default)]
pub struct Context {
    pub(crate) environment: Environment,
    pub(crate) volatile: Volatile,
}

#[derive(Debug)]
pub enum Constraint {
    ClassAssertion(usize, Assertion),
    MatchDeep(usize, SmolStr, SmolStr),
    MatchSolve(usize, SmolStr, TyIdx),
    UnifyDeep(usize, usize),
    UnifySolve(usize, TyIdx),
}
