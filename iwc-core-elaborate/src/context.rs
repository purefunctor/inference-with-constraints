use std::collections::HashMap;

use concurrent_queue::ConcurrentQueue;
use iwc_arena::Arena;
use iwc_core_ast::{
    expr::Expr,
    ty::{Instance, Type, TypeIdx},
};
use iwc_core_constraint::Constraint;
use smol_str::SmolStr;

#[derive(Default)]
pub struct Environment {
    pub(crate) constructors: HashMap<SmolStr, TypeIdx>,
    pub(crate) values: HashMap<SmolStr, TypeIdx>,
    #[allow(dead_code)]
    pub(crate) instances: HashMap<SmolStr, Vec<Instance>>,
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
