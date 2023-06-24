//! Implements operations on the environment.

use anyhow::Context;
use iwc_core_ast::ty::TyIdx;

impl super::Context {
    pub fn lookup_variable(&self, name: &str) -> anyhow::Result<TyIdx> {
        self.bindings
            .get(name)
            .context(format!("Unbound variable {:?}", name))
            .copied()
    }

    pub fn with_bound_variable<F, T>(&mut self, name: &str, ty: TyIdx, action: F) -> T
    where
        F: FnOnce(&mut Self) -> T,
    {
        self.bindings.insert(name.into(), ty);
        let result = action(self);
        self.bindings.remove(name);
        result
    }
}
