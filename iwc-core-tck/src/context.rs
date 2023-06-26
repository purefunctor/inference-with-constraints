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
    ty::{Assertion, Ty, TyIdx},
};
use smol_str::SmolStr;

pub struct Environment {
    bindings: HashMap<SmolStr, TyIdx>,
}

pub struct Volatile {
    expr_arena: Arena<Expr>,
    ty_arena: Arena<Ty>,
    fresh_index: usize,
    constraints: Vec<Constraint>,
}

pub struct Context {
    environment: Environment,
    volatile: Volatile,
}

pub enum Constraint {
    ClassAssertion(usize, Assertion),
    UnifyDeep(usize, usize),
    UnifySolve(usize, TyIdx),
}
