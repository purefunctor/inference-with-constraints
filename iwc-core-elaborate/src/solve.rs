use std::collections::HashMap;

use iwc_core_ast::ty::TypeIdx;
use iwc_core_constraint::Constraint;
use iwc_core_error::UnifyError;

use crate::{context::Context, unify::Unify};

pub struct Solve<'context> {
    context: &'context mut Context,
    unification_solved: HashMap<usize, TypeIdx>,
    unification_deferred: Vec<(usize, usize)>,
    unification_errors: Vec<UnifyError>,
}

impl<'context> Solve<'context> {
    pub fn new(context: &'context mut Context) -> Self {
        Self {
            context,
            unification_solved: HashMap::new(),
            unification_deferred: Vec::new(),
            unification_errors: Vec::new(),
        }
    }

    pub fn as_unify<'solve>(&'solve mut self) -> Unify<'solve> {
        Unify::new(self.context)
    }

    pub(crate) fn step(&mut self, constraint: Constraint) {
        match constraint {
            Constraint::ClassEntail(_) => todo!(),
            Constraint::ClassEvidence(_) => todo!(),
            Constraint::UnifyDeep(t_name, u_name) => {
                let t_idx = self.unification_solved.get(&t_name).copied();
                let u_idx = self.unification_solved.get(&u_name).copied();
                match (t_idx, u_idx) {
                    (Some(t_idx), Some(u_idx)) => {
                        self.as_unify().unify(t_idx, u_idx);
                    }
                    (None, Some(u_idx)) => {
                        self.unification_solved.insert(t_name, u_idx);
                    }
                    (Some(t_idx), None) => {
                        self.unification_solved.insert(u_name, t_idx);
                    }
                    (None, None) => {
                        self.unification_deferred.push((u_name, t_name));
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

        self.unification_deferred.retain(|(t_name, u_name)| {
            let t_idx = self.unification_solved.get(t_name).copied();
            let u_idx = self.unification_solved.get(u_name).copied();
            if t_idx.is_some() || u_idx.is_some() {
                self.context
                    .constraints
                    .push(Constraint::UnifyDeep(*t_name, *u_name))
                    .unwrap();
                false
            } else {
                true
            }
        });
    }

    pub fn solve(&mut self) {
        while let Ok(constraint) = self.context.constraints.pop() {
            self.step(constraint)
        }
    }
}
