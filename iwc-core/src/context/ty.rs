use crate::types::{Ty, TyIdx};

use super::Context;

/// Helper functions for constructing types.
impl Context {
    pub fn ty_unit(&mut self) -> TyIdx {
        self.ty_arena.allocate(Ty::Unit)
    }

    pub fn ty_variable(&mut self, v: &str) -> TyIdx {
        self.ty_arena.allocate(Ty::Variable(v.into()))
    }

    pub fn ty_unification(&mut self, v: usize) -> TyIdx {
        self.ty_arena.allocate(Ty::Unification(v))
    }

    pub fn ty_function(&mut self, a: TyIdx, r: TyIdx) -> TyIdx {
        self.ty_arena.allocate(Ty::Function(a, r))
    }

    pub fn ty_pair(&mut self, a: TyIdx, b: TyIdx) -> TyIdx {
        self.ty_arena.allocate(Ty::Pair(a, b))
    }
}

/// Helper functions for compiler-constructed types.
impl Context {
    pub fn ty_unification_fresh(&mut self) -> TyIdx {
        let index = self.fresh_index;
        self.fresh_index += 1;
        self.ty_unification(index)
    }
}

/// Type traversals among other utilities.
impl Context {
    pub fn occurs_check(&self, t: TyIdx, u: usize) -> bool {
        match &self.ty_arena[t] {
            Ty::Unit => false,
            Ty::Variable(_) => false,
            Ty::Unification(v) => u == *v,
            Ty::Function(a, r) => self.occurs_check(*a, u) || self.occurs_check(*r, u),
            Ty::Pair(a, b) => self.occurs_check(*a, u) || self.occurs_check(*b, u),
        }
    }
}
