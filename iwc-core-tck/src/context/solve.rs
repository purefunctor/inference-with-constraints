//! Implements the inference constraint solver.

use std::collections::HashMap;

use iwc_core_ast::ty::{Assertion, TyIdx};

use super::{Constraint, Context};

type Unifications = HashMap<usize, TyIdx>;

impl Context {
    pub fn solve(&mut self) -> anyhow::Result<()> {
        let mut constraints = std::mem::take(&mut self.constraints);
        let mut unifications = Unifications::new();
        let mut unsolved_deep = vec![];

        loop {
            for constraint in &constraints {
                match constraint {
                    Constraint::ClassAssertion(Assertion { .. }) => {
                        unimplemented!("Entailment!");
                    }
                    Constraint::UnifyDeep(u, v) => {
                        let u_ty = unifications.get(u);
                        let v_ty = unifications.get(v);
                        match (u_ty, v_ty) {
                            (Some(u_ty), Some(v_ty)) => {
                                self.unify(*u_ty, *v_ty)?;
                            }
                            (None, Some(v_ty)) => {
                                unifications.insert(*u, *v_ty);
                            }
                            (Some(u_ty), None) => {
                                unifications.insert(*v, *u_ty);
                            }
                            (None, None) => {
                                unsolved_deep.push((*u, *v));
                            }
                        }
                    }
                    Constraint::UnifySolve(u, t) => {
                        unifications.insert(*u, *t);
                    }
                }
            }

            constraints = std::mem::take(&mut self.constraints);

            // We verify which constraints can make more progress,
            // but we defer solving them until the next iteration.
            unsolved_deep.retain(|(u, v)| {
                let u_ty = unifications.get(u);
                let v_ty = unifications.get(v);
                if u_ty.is_some() || v_ty.is_some() {
                    constraints.push(Constraint::UnifyDeep(*u, *v));
                    false
                } else {
                    true
                }
            });

            if constraints.is_empty() {
                break;
            }
        }

        Ok(())
    }
}
