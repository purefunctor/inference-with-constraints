pub mod check;
pub mod infer;
pub mod instantiate;
pub mod unify;

use std::collections::HashMap;

use anyhow::Context;
use iwc_arena::Arena;
use iwc_core_ast::{
    expr::Expr,
    ty::{Instance, Type, TypeIdx},
};
use iwc_core_constraint::Constraint;
use smol_str::SmolStr;

#[derive(Default)]
pub struct Environment {
    constructors: HashMap<SmolStr, TypeIdx>,
    values: HashMap<SmolStr, TypeIdx>,
    instances: HashMap<SmolStr, Vec<Instance>>,
}

impl Environment {
    pub fn lookup_value(&mut self, key: &str) -> anyhow::Result<TypeIdx> {
        self.values
            .get(key)
            .context(format!("No binding found {:?}", key))
            .copied()
    }

    pub fn insert_value(&mut self, key: &str, value: TypeIdx) {
        self.values.insert(key.into(), value);
    }

    pub fn remove_value(&mut self, key: &str) {
        self.values.remove(key);
    }

    pub fn lookup_constructor(&mut self, key: &str) -> anyhow::Result<TypeIdx> {
        self.constructors
            .get(key)
            .context(format!("No constructor found {:?}", key))
            .copied()
    }

    pub fn insert_constructor(&mut self, key: &str, value: TypeIdx) {
        self.constructors.insert(key.into(), value);
    }

    pub fn remove_constructor(&mut self, key: &str) {
        self.constructors.remove(key);
    }

    pub fn insert_instance(&mut self, key: &str, value: Instance) {
        self.instances
            .entry(key.into())
            .or_insert_with(|| vec![])
            .push(value);
    }

    pub fn lookup_instance(&mut self, key: &str) -> anyhow::Result<&Vec<Instance>> {
        self.instances
            .get(key)
            .context(format!("No instance found {:?}", key))
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
