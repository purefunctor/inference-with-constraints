use std::collections::HashMap;

use crate::types::{Constraint, TyIdx};

use super::Context;

/// The core constraint-solving algorithm.
///
/// This algoritihm takes the [`Constraint`]s that have been generated so far
/// and performs the necessary steps to solve them. Different constraints may
/// emit different metadata—for instance, unification constraints may insert
/// solutions for unification variables, while for instance constraints, they
/// can return the necessary instance dictionaries to be inserted at the value
/// level.
///
/// At a high-level, the [`Context::solve`] function operates by iteratively
/// solving constraints. It also takes into account queueing newly generated
/// constraints, as well as deferring constraints that require further context
/// before being solved.
impl Context {
    pub fn solve(&mut self) -> anyhow::Result<()> {
        let mut constraints = std::mem::take(&mut self.constraints);
        let mut unifications: HashMap<usize, TyIdx> = HashMap::new();
        let mut unsolved_deep = vec![];

        loop {
            for constraint in &constraints {
                match constraint {
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
