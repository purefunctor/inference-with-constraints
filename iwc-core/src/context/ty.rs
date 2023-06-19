use crate::types::{DeBrujin, Rank, Ty, TyIdx};

use super::Context;

/// Helper functions for constructing types.
impl Context {
    pub fn ty_unit(&mut self) -> TyIdx {
        self.ty_arena.allocate(Ty::Unit)
    }

    pub fn ty_variable(&mut self, v: DeBrujin, r: Rank) -> TyIdx {
        self.ty_arena.allocate(Ty::Variable(v, r))
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

    pub fn ty_forall(&mut self, v: DeBrujin, r: Rank, t: TyIdx) -> TyIdx {
        self.ty_arena.allocate(Ty::Forall(v, r, t))
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
            Ty::Variable(_, _) => false,
            Ty::Unification(v) => u == *v,
            Ty::Function(a, r) => self.occurs_check(*a, u) || self.occurs_check(*r, u),
            Ty::Pair(a, b) => self.occurs_check(*a, u) || self.occurs_check(*b, u),
            Ty::Forall(_, _, t) => self.occurs_check(*t, u),
        }
    }

    pub fn instantiate_type(&mut self, t: TyIdx) -> TyIdx {
        if let Ty::Forall(vs, r, t) = &self.ty_arena[t] {
            let vs = *vs;
            let r = *r;
            let t = *t;

            self.instantiate_type_core(vs, r, t)
        } else {
            t
        }
    }

    fn instantiate_type_core(&mut self, vs: DeBrujin, r: Rank, t: TyIdx) -> TyIdx {
        match &self.ty_arena[t] {
            Ty::Unit => t,
            Ty::Variable(v, s) => {
                if *s == r && *v < vs {
                    self.ty_unification_fresh()
                } else {
                    t
                }
            }
            Ty::Unification(_) => t,
            Ty::Function(f, x) => {
                let f = *f;
                let x = *x;

                let f = self.instantiate_type_core(vs, r, f);
                let x = self.instantiate_type_core(vs, r, x);

                self.ty_function(f, x)
            }
            Ty::Pair(a, b) => {
                let a = *a;
                let b = *b;

                let a = self.instantiate_type_core(vs, r, a);
                let b = self.instantiate_type_core(vs, r, b);

                self.ty_pair(a, b)
            }
            Ty::Forall(_, _, _) => t,
        }
    }
}
