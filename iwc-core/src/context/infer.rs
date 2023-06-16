use anyhow::Context;

use crate::types::{Expr, ExprIdx, TyIdx};

impl super::Context {
    pub fn infer(&mut self, e_idx: ExprIdx) -> anyhow::Result<TyIdx> {
        match &self.ex_arena[e_idx] {
            Expr::Unit => Ok(self.ty_unit()),
            Expr::Variable(v) => self
                .lookup_type(&v)
                .context(format!("Unbound variable {:?}", v)),
            Expr::Lambda(x, v) => {
                let x = x.clone();
                let v = *v;

                let x_t = self.ty_unification_fresh();
                let v_t = self.with_bound_type(&x, x_t, |context| context.infer(v))?;

                Ok(self.ty_function(x_t, v_t))
            }
            Expr::Application(f, x) => {
                let f = *f;
                let x = *x;

                let f_ty = self.infer(f)?;
                let x_ty = self.infer(x)?;
                let r_ty = self.ty_unification_fresh();
                let i_ty = self.ty_function(x_ty, r_ty);

                self.unify(f_ty, i_ty)?;

                Ok(r_ty)
            }
            Expr::Pair(a, b) => {
                let a = *a;
                let b = *b;

                let a_ty = self.infer(a)?;
                let b_ty = self.infer(b)?;

                Ok(self.ty_pair(a_ty, b_ty))
            }
        }
    }
}
