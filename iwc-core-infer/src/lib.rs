pub mod check;
pub mod infer;
pub mod instantiate;
pub mod unify;

use std::collections::HashMap;

use anyhow::Context;
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

impl Environment {
    pub fn lookup_value_binding(&mut self, key: &str) -> anyhow::Result<TypeIdx> {
        self.value_bindings
            .get(key)
            .context(format!("No binding found {:?}", key))
            .copied()
    }

    pub fn insert_value_binding(&mut self, key: &str, value: TypeIdx) {
        self.value_bindings.insert(key.into(), value);
    }

    pub fn remove_value_binding(&mut self, key: &str) {
        self.value_bindings.remove(key);
    }

    pub fn lookup_constructor_binding(&mut self, key: &str) -> anyhow::Result<TypeIdx> {
        self.constructor_bindings
            .get(key)
            .context(format!("No constructor found {:?}", key))
            .copied()
    }

    pub fn insert_constructor_binding(&mut self, key: &str, value: TypeIdx) {
        self.constructor_bindings.insert(key.into(), value);
    }

    pub fn remove_constructor_binding(&mut self, key: &str) {
        self.constructor_bindings.remove(key);
    }
}

#[derive(Default)]
pub struct Volatile {
    pub expr_arena: Arena<Expr>,
    pub type_arena: Arena<Type>,
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
    pub environment: Environment,
    pub volatile: Volatile,
    constraints: Vec<Constraint>,
}

impl Infer {
    pub fn take_constraints(&mut self) -> Vec<Constraint> {
        std::mem::take(&mut self.constraints)
    }
}
