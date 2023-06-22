use anyhow::bail;

use crate::types::{Constraint, Ty, TyIdx};

use super::Context;

/// The core unification algorithm.
///
/// This algorithm is as simple as unification algorithms can get, but rather
/// than eagerly emitting substitutions, it pushes [`Constraint`]s onto the
/// [`Context`] which are solved later.
impl Context {
    pub fn unify(&mut self, t_idx: TyIdx, u_idx: TyIdx) -> anyhow::Result<()> {
        match (&self.ty_arena[t_idx], &self.ty_arena[u_idx]) {
            // Trivial
            (Ty::Unit, Ty::Unit) => (),
            // Identity
            (Ty::Variable { name: a, rank: r }, Ty::Variable { name: b, rank: s })
                if a == b && r == s =>
            {
                ()
            }
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

    fn emit_deep(&mut self, u: usize, v: usize) {
        self.constraints.push(Constraint::UnifyDeep(u, v));
    }

    fn emit_solve(&mut self, u: usize, t: TyIdx) {
        self.constraints.push(Constraint::UnifySolve(u, t));
    }
}
