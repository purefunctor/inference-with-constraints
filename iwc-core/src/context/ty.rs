use smol_str::SmolStr;

use crate::types::{Ty, TyIdx, TypeVariableBindings};

use super::Context;

/// Helper functions for constructing types.
impl Context {
    pub fn ty_unit(&mut self) -> TyIdx {
        self.ty_arena.allocate(Ty::Unit)
    }

    pub fn ty_variable(&mut self, v: &str, r: usize) -> TyIdx {
        self.ty_arena.allocate(Ty::Variable {
            name: v.into(),
            rank: r,
        })
    }

    pub fn ty_unification(&mut self, v: usize) -> TyIdx {
        self.ty_arena.allocate(Ty::Unification { value: v })
    }

    pub fn ty_function(&mut self, a: TyIdx, r: TyIdx) -> TyIdx {
        self.ty_arena.allocate(Ty::Function {
            argument: a,
            result: r,
        })
    }

    pub fn ty_pair(&mut self, a: TyIdx, b: TyIdx) -> TyIdx {
        self.ty_arena.allocate(Ty::Pair { left: a, right: b })
    }

    pub fn ty_forall(&mut self, vs: &[&str], r: usize, t: TyIdx) -> TyIdx {
        self.ty_arena.allocate(Ty::Forall {
            variables: vs.iter().map(SmolStr::new).collect(),
            rank: r,
            ty: t,
        })
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
                variables: _,
                rank: _,
                ty: _,
            } => ty,
            Ty::Constrained {
                assertions: _,
                ty: _,
            } => ty,
        }
    }
}
