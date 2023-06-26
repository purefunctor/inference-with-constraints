//! Implements common operations on the context.

use iwc_core_ast::ty::{Ty, TyIdx};

use super::Context;

impl Context {
    pub fn fresh_marker(&mut self) -> usize {
        let value = self.volatile.fresh_index;
        self.volatile.fresh_index += 1;
        value
    }

    pub fn fresh_unification_variable(&mut self) -> TyIdx {
        let value = self.fresh_marker();
        self.volatile.ty_arena.allocate(Ty::Unification { value })
    }
}
