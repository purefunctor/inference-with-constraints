use crate::types::{Assertions, Ty, TyIdx, TypeVariableBindings};

use super::Context;

/// Helper functions for constructing types.
impl Context {
    pub fn ty_unit(&mut self) -> TyIdx {
        self.ty_arena.allocate(Ty::Unit)
    }

    pub fn ty_variable(&mut self, name: &str, rank: usize) -> TyIdx {
        self.ty_arena.allocate(Ty::Variable {
            name: name.into(),
            rank,
        })
    }

    pub fn ty_unification(&mut self, value: usize) -> TyIdx {
        self.ty_arena.allocate(Ty::Unification { value })
    }

    pub fn ty_function(&mut self, argument: TyIdx, result: TyIdx) -> TyIdx {
        self.ty_arena.allocate(Ty::Function { argument, result })
    }

    pub fn ty_pair(&mut self, left: TyIdx, right: TyIdx) -> TyIdx {
        self.ty_arena.allocate(Ty::Pair { left, right })
    }

    pub fn ty_forall(&mut self, variables: TypeVariableBindings, rank: usize, ty: TyIdx) -> TyIdx {
        self.ty_arena.allocate(Ty::Forall {
            variables,
            rank,
            ty,
        })
    }

    pub fn ty_constrained(&mut self, assertions: Assertions, ty: TyIdx) -> TyIdx {
        self.ty_arena.allocate(Ty::Constrained { assertions, ty })
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

/// Well-formedness checks.
impl Context {
    pub fn occurs_check(&self, t: TyIdx, u: usize) -> bool {
        match &self.ty_arena[t] {
            Ty::Unit => false,
            Ty::Variable { name: _, rank: _ } => false,
            Ty::Unification { value: v } => u == *v,
            Ty::Function {
                argument: a,
                result: r,
            } => self.occurs_check(*a, u) || self.occurs_check(*r, u),
            Ty::Pair { left: a, right: b } => self.occurs_check(*a, u) || self.occurs_check(*b, u),
            Ty::Forall {
                variables: _,
                rank: _,
                ty: t,
            } => self.occurs_check(*t, u),
            Ty::Constrained {
                assertions: _,
                ty: t,
            } => self.occurs_check(*t, u),
        }
    }
}
