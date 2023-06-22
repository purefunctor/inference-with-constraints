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

/// Type traversals among other utilities.
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

    pub fn instantiate_type(&mut self, t: TyIdx) -> TyIdx {
        if let Ty::Forall {
            variables,
            rank,
            ty,
        } = &self.ty_arena[t]
        {
            // NOTE: TinyVec/SmolStr makes it so that sufficiently
            // small data does not require heap allocations--in the
            // general case, clones such as these are inexpensive.
            let variables = variables.clone();
            let rank = *rank;
            let ty = *ty;

            self.instantiate_type_core(&variables, rank, ty)
        } else {
            t
        }
    }

    fn instantiate_type_core(
        &mut self,
        variables: &TypeVariableBindings,
        rank: usize,
        ty: TyIdx,
    ) -> TyIdx {
        match &self.ty_arena[ty] {
            Ty::Unit => ty,
            Ty::Variable { name: v, rank: s } => {
                if *s == rank && variables.contains(v) {
                    self.ty_unification_fresh()
                } else {
                    ty
                }
            }
            Ty::Unification { value: _ } => ty,
            Ty::Function {
                argument: a,
                result: r,
            } => {
                let a = *a;
                let r = *r;

                let a = self.instantiate_type_core(variables, rank, a);
                let r = self.instantiate_type_core(variables, rank, r);

                self.ty_function(a, r)
            }
            Ty::Pair { left: a, right: b } => {
                let a = *a;
                let b = *b;

                let a = self.instantiate_type_core(variables, rank, a);
                let b = self.instantiate_type_core(variables, rank, b);

                self.ty_pair(a, b)
            }
            Ty::Forall {
                variables: inner_variables,
                rank: inner_rank,
                ty: inner_ty,
            } => {
                debug_assert_ne!(rank, *inner_rank, "Malformed polymorphic type.");

                let inner_variables = inner_variables.clone();
                let inner_rank = *inner_rank;
                let inner_ty = self.instantiate_type_core(variables, rank, *inner_ty);

                self.ty_forall(inner_variables, inner_rank, inner_ty)
            }
            Ty::Constrained { assertions, ty } => {
                let assertions = assertions.clone();
                let ty = self.instantiate_type_core(variables, rank, *ty);

                self.ty_constrained(assertions, ty)
            }
        }
    }
}
