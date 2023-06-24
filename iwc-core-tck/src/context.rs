pub mod common;
pub mod instantiate;
pub mod unify;

use iwc_arena::Arena;
use iwc_core_ast::ty::{Ty, TyIdx};

pub struct Context {
    ty_arena: Arena<Ty>,
    fresh_index: usize,
    constraints: Vec<Constraint>,
}

pub enum Constraint {
    UnifyDeep(usize, usize),
    UnifySolve(usize, TyIdx),
}
