use std::{
    collections::{HashMap, HashSet},
    iter::zip,
};

use iwc_core_ast::ty::{pretty::pretty_print_assertions, Assertion, TypeIdx};
use iwc_core_constraint::Constraint;
use iwc_core_error::UnifyError;

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
    pub(crate) entailment_evidences: HashMap<usize, Evidence>,
    pub(crate) entailment_deferred: Vec<DeferredAssertion>,
}

#[derive(Debug)]
pub struct DeferredAssertion {
    index: usize,
    assertion: Assertion,
    needs_solution: HashSet<(usize, usize)>,
}

impl<'context> Solve<'context> {
    pub fn new(context: &'context mut Context) -> Self {
        Self {
            context,
            unification_solved: HashMap::new(),
            unification_deferred: Vec::new(),
            unification_errors: Vec::new(),
            entailment_evidences: HashMap::new(),
            entailment_deferred: Vec::new(),
        }
    }

    pub(crate) fn step(&mut self) {
        while let Ok(constraint) = self.context.constraints.pop() {
            match constraint {
                Constraint::ClassEntail(index, assertion) => {
                    let a = pretty_print_assertions(
                        &self.context.volatile.type_arena,
                        &[assertion.clone()],
                    );
                    println!("entail: {a}");
                    match Entail::new(self.context).entail(&assertion) {
                        EntailResult::Solved {
                            evidence,
                            instance_assertion,
                        } => {
                            println!("{a} is solved.");
                            self.entailment_evidences.insert(index, evidence);
                            for (t_idx, u_idx) in
                                zip(&assertion.arguments, &instance_assertion.arguments)
                            {
                                Unify::new(self.context).unify(*t_idx, *u_idx);
                            }
                        }
                        EntailResult::Depends {
                            evidence,
                            instance_assertion,
                            instance_dependencies,
                        } => {
                            println!("{a} requires dependencies.");
                            self.entailment_evidences.insert(index, evidence);
                            for (t_idx, u_idx) in
                                zip(&assertion.arguments, &instance_assertion.arguments)
                            {
                                Unify::new(self.context).unify(*t_idx, *u_idx);
                            }
                            for (dependency_index, dependency_assertion) in instance_dependencies {
                                self.context
                                    .constraints
                                    .push(Constraint::ClassEntail(
                                        dependency_index,
                                        dependency_assertion,
                                    ))
                                    .unwrap();
                            }
                        }
                        EntailResult::Deferred { needs_solution } => {
                            println!("{a} is deferred.");
                            self.entailment_deferred.push(DeferredAssertion {
                                index,
                                assertion,
                                needs_solution,
                            });
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

        let entailment_deferred = std::mem::take(&mut self.entailment_deferred);
        for DeferredAssertion {
            index,
            mut assertion,
            mut needs_solution,
        } in entailment_deferred
        {
            needs_solution.retain(|(index, name)| {
                if let Some(value) = self.unification_solved.get(name) {
                    assertion.arguments[*index] = *value;
                    false
                } else {
                    true
                }
            });

            if needs_solution.is_empty() {
                self.context
                    .constraints
                    .push(Constraint::ClassEntail(index, assertion))
                    .unwrap();
            } else {
                self.entailment_deferred.push(DeferredAssertion {
                    index,
                    assertion,
                    needs_solution,
                });
            }
        }
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
