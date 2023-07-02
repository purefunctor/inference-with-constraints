pub mod instantiate;
pub mod unify;

use std::collections::HashMap;

use iwc_arena::Arena;
use iwc_core_ast::{
    expr::Expr,
    ty::{Type, TypeIdx},
};
use iwc_core_constraint::Constraint;
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

impl Volatile {
    pub fn fresh_unification(&mut self) -> TypeIdx {
        let name = self.fresh_index;
        self.fresh_index += 1;
        self.type_arena.allocate(Type::Unification { name })
    }
}

#[derive(Default)]
pub struct Infer {
    environment: Environment,
    volatile: Volatile,
    constraints: Vec<Constraint>,
}
