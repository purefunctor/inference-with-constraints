use crate::types::TyIdx;

use super::Context;

/// Helper functions for modifying the environment.
impl Context {
    pub fn lookup_type(&self, k: &str) -> Option<TyIdx> {
        self.ty_bindings.get(k).copied()
    }

    pub fn bind_type(&mut self, k: &str, v: TyIdx) {
        self.ty_bindings.insert(k.into(), v);
    }

    pub fn unbind_type(&mut self, k: &str) {
        self.ty_bindings.remove(k);
    }

    /// Temporarily binds a type before performing an action.
    pub fn with_bound_type<F, T>(&mut self, k: &str, v: TyIdx, f: F) -> T
    where
        F: FnOnce(&mut Self) -> T,
    {
        self.bind_type(k, v);
        let result = f(self);
        self.unbind_type(k);
        result
    }
}
