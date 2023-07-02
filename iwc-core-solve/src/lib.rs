use std::collections::HashMap;

use iwc_core_ast::ty::TypeIdx;
use iwc_core_constraint::{Constraint, UnifyError};
use iwc_core_infer::Infer;

pub struct Solve {
    infer: Infer,
    unification_solved: HashMap<usize, TypeIdx>,
    unification_unsolved: Vec<(usize, usize)>,
    unification_errors: Vec<UnifyError>,
}

impl Solve {
    pub(crate) fn step(&mut self, constraints: Vec<Constraint>) -> Vec<Constraint> {
        for constraint in constraints {
            match constraint {
                Constraint::ClassAssertion(_) => unimplemented!("ClassAssertion"),
                Constraint::UnifyDeep(u_name, t_name) => {
                    let u_ty = self.unification_solved.get(&u_name);
                    let t_ty = self.unification_solved.get(&t_name);
                    match (u_ty, t_ty) {
                        (Some(u_ty), Some(t_ty)) => {
                            self.infer.unify(*u_ty, *t_ty);
                        }
                        (None, Some(t_ty)) => {
                            self.unification_solved.insert(u_name, *t_ty);
                        }
                        (Some(u_ty), None) => {
                            self.unification_solved.insert(t_name, *u_ty);
                        }
                        (None, None) => {
                            self.unification_unsolved.push((u_name, t_name));
                        }
                    }
                }
                Constraint::UnifySolve(name, ty) => {
                    self.unification_solved.insert(name, ty);
                }
                Constraint::UnifyError(error) => {
                    self.unification_errors.push(error);
                }
            }
        }

        let mut constraints = self.infer.take_constraints();

        self.unification_unsolved.retain(|(u_name, t_name)| {
            let u_ty = self.unification_solved.get(u_name);
            let t_ty = self.unification_solved.get(t_name);
            if u_ty.is_some() || t_ty.is_some() {
                constraints.push(Constraint::UnifyDeep(*u_name, *t_name));
                false
            } else {
                true
            }
        });

        constraints
    }

    pub fn solve(&mut self) {
        let mut constraints = self.infer.take_constraints();
        loop {
            constraints = self.step(constraints);
            if constraints.is_empty() {
                break;
            }
        }
    }
}
