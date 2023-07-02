use std::collections::HashMap;

use iwc_arena::Arena;
use iwc_core_ast::{
    expr::Expr,
    ty::{Type, TypeIdx},
};
use iwc_core_constraints::Constraint;
use smol_str::SmolStr;

#[derive(Default)]
pub struct Environment {
    constructor_bindings: HashMap<SmolStr, TypeIdx>,
    value_bindings: HashMap<SmolStr, TypeIdx>,
}

#[derive(Default)]
pub struct Volatile {
    expr_arena: Arena<Expr>,
    type_arena: Arena<Type>,
    fresh_index: usize,
}

#[derive(Default)]
pub struct Infer {
    environment: Environment,
    volatile: Volatile,
    constraints: Vec<Constraint>,
}
