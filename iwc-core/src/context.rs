use std::collections::HashMap;

use iwc_arena::Arena;
use smol_str::SmolStr;

mod env;
mod expr;
mod infer;
mod solve;
mod ty;
mod unify;

use crate::types::{Constraint, Expr, Ty, TyIdx};

/// Keeps track of the state used throughout the inference algorithm.
///
/// Rather than defining functions to accept the state as an argument, we
/// instead define them as methods for the [`Context`] struct.
#[derive(Default)]
pub struct Context {
    pub(crate) ty_arena: Arena<Ty>,
    ex_arena: Arena<Expr>,
    ty_bindings: HashMap<SmolStr, TyIdx>,
    fresh_index: usize,
    pub(crate) constraints: Vec<Constraint>,
}
