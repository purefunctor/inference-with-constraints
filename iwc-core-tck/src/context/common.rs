//! Implements common operations on the context.

use iwc_core_ast::ty::{Ty, TyIdx};

use super::Context;

impl Context {
    pub fn fresh_unification_variable(&mut self) -> TyIdx {
        let value = self.fresh_index;
        let ty = self.ty_arena.allocate(Ty::Unification { value });
        self.fresh_index += 1;
        ty
    }
}
