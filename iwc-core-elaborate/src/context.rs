use std::collections::HashMap;

use concurrent_queue::ConcurrentQueue;
use iwc_arena::Arena;
use iwc_core_ast::{
    expr::Expr,
    ty::{Class, Instance, Type, TypeIdx},
};
use iwc_core_constraint::Constraint;
use smol_str::SmolStr;

#[derive(Default)]
pub struct Environment {
    pub(crate) constructors: HashMap<SmolStr, TypeIdx>,
    pub(crate) values: HashMap<SmolStr, TypeIdx>,
    pub(crate) classes: HashMap<SmolStr, Class>,
    pub(crate) instances: HashMap<SmolStr, Vec<Instance>>,
}

impl Environment {
    pub fn find_instances(&self, name: &str) -> Vec<Instance> {
        self.instances.get(name).cloned().unwrap_or(vec![])
    }
}

#[derive(Default)]
pub struct Volatile {
    pub(crate) expr_arena: Arena<Expr>,
    pub(crate) type_arena: Arena<Type>,
}

pub struct Context {
    pub(crate) environment: Environment,
    pub(crate) volatile: Volatile,
    pub(crate) fresh: usize,
    pub(crate) constraints: ConcurrentQueue<Constraint>,
}

impl Context {
    pub fn fresh_index(&mut self) -> usize {
        let index = self.fresh;
        self.fresh += 1;
        index
    }

    pub fn fresh_unification(&mut self) -> TypeIdx {
        let name = self.fresh;
        self.fresh += 1;
        self.volatile
            .type_arena
            .allocate(Type::Unification { name })
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            environment: Environment::default(),
            volatile: Volatile::default(),
            fresh: 0,
            constraints: ConcurrentQueue::bounded(512),
        }
    }
}
