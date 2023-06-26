//! Implements the unification algorithm.
//!
//! Unification determines whether two types are equal, emitting substitutions for unification
//! variables. The inference constraint solver works with these substitutions to iteratively
//! solve unification variables, and unify types (which may lead to more substitutions) until
//! no more progress can be made.

use anyhow::bail;
use iwc_core_ast::ty::{Ty, TyIdx};

use super::{Constraint, Context};

impl Context {
    pub fn unify(&mut self, t_idx: TyIdx, u_idx: TyIdx) -> anyhow::Result<()> {
        match (
            &self.volatile.ty_arena[t_idx],
            &self.volatile.ty_arena[u_idx],
        ) {
            // Trivial
            (Ty::Unit, Ty::Unit) => (),
            // Identity
            (Ty::Variable { name: a, rank: r }, Ty::Variable { name: b, rank: s })
                if a == b && r == s => {}
            (Ty::Unification { value: a }, Ty::Unification { value: b }) => {
                if a != b {
                    self.emit_deep(*a, *b);
                }
            }
            // Unification
            (t, Ty::Unification { value: u }) => {
                if t.is_polymorphic() {
                    bail!("Impredicative types.");
                }
                if self.occurs_check(t_idx, *u) {
                    bail!("Infinite type occurred.");
                }
                self.emit_solve(*u, t_idx);
            }
            (Ty::Unification { value: t }, u) => {
                if u.is_polymorphic() {
                    bail!("Impredicative types.");
                }
                if self.occurs_check(u_idx, *t) {
                    bail!("Infinite type occurred.");
                }
                self.emit_solve(*t, u_idx);
            }
            // Compound types
            (
                Ty::Function {
                    argument: a,
                    result: r,
                },
                Ty::Function {
                    argument: b,
                    result: s,
                },
            ) => {
                let a = *a;
                let b = *b;

                let r = *r;
                let s = *s;

                self.unify(a, b)?;
                self.unify(r, s)?;
            }
            (Ty::Pair { left: a, right: b }, Ty::Pair { left: x, right: y }) => {
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

    fn occurs_check(&self, t: TyIdx, u: usize) -> bool {
        match &self.volatile.ty_arena[t] {
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

    fn emit_deep(&mut self, u: usize, v: usize) {
        self.volatile.constraints.push(Constraint::UnifyDeep(u, v));
    }

    fn emit_solve(&mut self, u: usize, t: TyIdx) {
        self.volatile.constraints.push(Constraint::UnifySolve(u, t));
    }
}
