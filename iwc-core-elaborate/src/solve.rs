use std::collections::HashMap;

use iwc_core_ast::ty::{Assertion, Instance, TypeIdx};
use iwc_core_constraint::Constraint;
use iwc_core_error::UnifyError;
use smol_str::SmolStr;

use crate::{
    context::Context,
    entail::{Entail, EntailResult, Evidence},
    unify::Unify,
};

pub struct Solve<'context> {
    pub(crate) context: &'context mut Context,
    pub(crate) unification_solved: HashMap<usize, TypeIdx>,
    pub(crate) unification_deferred: Vec<(usize, usize)>,
    pub(crate) unification_errors: Vec<UnifyError>,
    pub(crate) entailment_evidence: HashMap<usize, Evidence>,
    pub(crate) entailment_instance: HashMap<usize, Instance>,
    pub(crate) entailment_substitution: HashMap<usize, HashMap<SmolStr, TypeIdx>>,
    pub(crate) entailment_deferred: Vec<(usize, Assertion)>,
}

impl<'context> Solve<'context> {
    pub fn new(context: &'context mut Context) -> Self {
        Self {
            context,
            unification_solved: HashMap::new(),
            unification_deferred: Vec::new(),
            unification_errors: Vec::new(),
            entailment_evidence: HashMap::new(),
            entailment_deferred: Vec::new(),
            entailment_substitution: HashMap::new(),
            entailment_instance: HashMap::new(),
        }
    }

    pub(crate) fn step(&mut self) {
        while let Ok(constraint) = self.context.constraints.pop() {
            match constraint {
                Constraint::ClassEntail(index, assertion) => {
                    // TODO: apply the substitution and solve unification variables

                    let result = match self.entailment_instance.get(&index) {
                        Some(instance) => {
                            // NOTE: we only need to mutably borrow the context.
                            Entail::new(self.context)
                                .entail_with(&assertion, instance)
                                .unwrap_or(EntailResult::Deferred)
                        }
                        None => Entail::new(self.context).entail(&assertion),
                    };

                    match result {
                        EntailResult::Solved { evidence } => {
                            self.entailment_evidence.insert(index, evidence);
                        }
                        EntailResult::Depends {
                            evidence,
                            instance,
                            substitution,
                        } => {
                            self.entailment_evidence.insert(index, evidence);
                            self.entailment_instance.insert(index, instance);
                            self.entailment_substitution.insert(index, substitution);
                        }
                        EntailResult::Deferred => {
                            self.entailment_deferred.push((index, assertion));
                        }
                    }
                }
                Constraint::UnifyDeep(t_name, u_name) => {
                    let t_idx = self.unification_solved.get(&t_name).copied();
                    let u_idx = self.unification_solved.get(&u_name).copied();
                    match (t_idx, u_idx) {
                        (Some(t_idx), Some(u_idx)) => {
                            Unify::new(self.context).unify(t_idx, u_idx);
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
