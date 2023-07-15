use std::collections::HashMap;

use iwc_core_ast::ty::{Assertion, Instance, Type, TypeIdx};
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

        let entailment_deferred = std::mem::take(&mut self.entailment_deferred);
        for (index, mut assertion) in entailment_deferred {
            let context = &mut self.context;
            let solution = &self.unification_solved;

            let default_substitution = HashMap::new();
            let substitution = self
                .entailment_substitution
                .get(&index)
                .unwrap_or(&default_substitution);

            let normalized = assertion.arguments.iter_mut().all(|argument| {
                let (normalized, replacement) =
                    normalize_argument(context, solution, substitution, *argument);

                *argument = replacement;

                normalized
            });

            if normalized {
                self.context
                    .constraints
                    .push(Constraint::ClassEntail(index, assertion))
                    .unwrap();
            } else {
                self.entailment_deferred.push((index, assertion));
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

fn normalize_argument(
    context: &mut Context,
    solution: &HashMap<usize, TypeIdx>,
    substitution: &HashMap<SmolStr, TypeIdx>,
    t_idx: TypeIdx,
) -> (bool, TypeIdx) {
    match &context.volatile.type_arena[t_idx] {
        Type::Constructor { .. } => (true, t_idx),
        Type::Variable { name, .. } => {
            if let Some(argument) = substitution.get(name) {
                (true, *argument)
            } else {
                (false, t_idx)
            }
        }
        Type::Unification { name } => {
            if let Some(argument) = solution.get(name) {
                (true, *argument)
            } else {
                (false, t_idx)
            }
        }
        Type::Function { arguments, result } => {
            let arguments = arguments.clone();
            let result = *result;

            let (arguments_normalized, arguments): (Vec<_>, Vec<_>) = arguments
                .iter()
                .map(|argument| normalize_argument(context, solution, substitution, *argument))
                .unzip();

            let (result_normalized, result) =
                normalize_argument(context, solution, substitution, result);

            (
                arguments_normalized.into_iter().all(|x| x) && result_normalized,
                context
                    .volatile
                    .type_arena
                    .allocate(Type::Function { arguments, result }),
            )
        }
        Type::Application { function, argument } => {
            let function = *function;
            let argument = *argument;

            let (function_normalized, function) =
                normalize_argument(context, solution, substitution, function);
            let (argument_normalized, argument) =
                normalize_argument(context, solution, substitution, argument);

            (
                function_normalized && argument_normalized,
                context
                    .volatile
                    .type_arena
                    .allocate(Type::Application { function, argument }),
            )
        }
        Type::Forall { .. } => panic!("Invalid type in assertion."),
        Type::Constrained { .. } => panic!("Invalid type in assertion."),
    }
}
