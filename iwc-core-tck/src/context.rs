pub mod common;
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

pub struct Context {
    // Environment
    bindings: HashMap<SmolStr, TyIdx>,
    // Volatile
    expr_arena: Arena<Expr>,
    ty_arena: Arena<Ty>,
    fresh_index: usize,
    // Accumulator
    constraints: Vec<Constraint>,
}

pub enum Constraint {
    ClassAssertion(Assertion),
    UnifyDeep(usize, usize),
    UnifySolve(usize, TyIdx),
}
