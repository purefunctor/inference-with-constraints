use anyhow::bail;

use crate::types::{Constraint, Ty, TyIdx};

use super::Context;

impl Context {
    pub fn unify(&mut self, t_idx: TyIdx, u_idx: TyIdx) -> anyhow::Result<()> {
        match (&self.ty_arena[t_idx], &self.ty_arena[u_idx]) {
            // Trivial
            (Ty::Unit, Ty::Unit) => (),
            // Identity
            (Ty::Variable(a), Ty::Variable(b)) if a == b => (),
            (Ty::Unification(a), Ty::Unification(b)) if a == b => (),
            // Unification
            (_, Ty::Unification(v)) => {
                if self.occurs_check(t_idx, *v) {
                    bail!("Infinite type occurred.");
                }
                self.emit(t_idx, u_idx);
            }
            (Ty::Unification(v), _) => {
                if self.occurs_check(u_idx, *v) {
                    bail!("Infinite type occurred.");
                }
                self.emit(t_idx, u_idx);
            }
            // Compound types
            (Ty::Function(a, r), Ty::Function(b, s)) => {
                let a = *a;
                let b = *b;

                let r = *r;
                let s = *s;

                self.unify(a, b)?;
                self.unify(r, s)?;
            }
            (Ty::Pair(a, b), Ty::Pair(x, y)) => {
                let a = *a;
                let x = *x;

                let b = *b;
                let y = *y;

                self.unify(a, x)?;
                self.unify(b, y)?;
            }
            // Failure
            (t_ty, u_ty) => {
                bail!("Failed to unify {:?} and {:?}", t_ty, u_ty);
            }
        }

        Ok(())
    }

    fn emit(&mut self, t: TyIdx, u: TyIdx) {
        self.constraints.push(Constraint::Unification(t, u));
    }
}
