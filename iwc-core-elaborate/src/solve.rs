use std::collections::HashMap;

use iwc_core_ast::ty::TypeIdx;
use iwc_core_constraint::Constraint;
use iwc_core_error::UnifyError;

use crate::{
    context::Context,
    entail::{Entail, EntailResult},
    unify::Unify,
};

pub struct Solve<'context> {
    pub(crate) context: &'context mut Context,
    pub(crate) unification_solved: HashMap<usize, TypeIdx>,
    pub(crate) unification_deferred: Vec<(usize, usize)>,
    pub(crate) unification_errors: Vec<UnifyError>,
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

    pub fn as_entail(&mut self) -> Entail {
        Entail::new(self.context)
    }

    pub(crate) fn step(&mut self) {
        while let Ok(constraint) = self.context.constraints.pop() {
            match constraint {
                Constraint::ClassEntail(_, assertion) => match self.as_entail().entail(assertion) {
                    EntailResult::Solved { evidence: _ } => todo!("Solved!"),
                    EntailResult::Depends {
                        evidence: _,
                        substitutions: _,
                    } => todo!("Dependent!"),
                    EntailResult::Deferred => todo!("Deferred!"),
                },
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
                            // Avoids infinite loops with unsolvable unifications.
                            self.unification_deferred.push((t_name, u_name));
                        }
                    }
                }
                Constraint::UnifySolve(t_name, u_idx) => {
                    self.unification_solved.insert(t_name, u_idx);
                }
                Constraint::UnifyError(error) => {
                    self.unification_errors.push(error);
                }
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
        loop {
            self.step();
            if self.context.constraints.is_empty() {
                break;
            }
        }
    }
}
