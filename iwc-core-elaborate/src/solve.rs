use std::collections::HashMap;

use iwc_core_ast::ty::{hash::hash_assertion, Assertion, Type, TypeIdx};
use iwc_core_constraint::Constraint;
use iwc_core_error::UnifyError;

use crate::{context::Context, unify::Unify};

// implementing entailment
//
// ClassEntail says that an assertion should be entailed given the current evidences.
// ClassEvidence says that an assertion should be assumed as an evidence
//
// ClassEvidence may have arguments that are unification variables, which are solved
// later. Every time a unification variable is solved, we need to take existing
// evidences and substitute them.
//
// We can record a mapping from unification variables to which evidences contain
// them, every time we solve a unification variable, we refer to this mapping
// to insert more evidences.
//
// The other solution is to make use of a reverse mapping where type indices map
// to unification variables that solved them. this way, we can "reverse" the
// substitution in that we can avoid adding more evidences.

/*

eqI :: forall a. Eq a => a -> Boolean
eqI a = eq [a] [a]

?0 -> Boolean

[Evidence(Eq, ?0)]

?1 -> ?1 -> Boolean

[Evidence(Eq, ?0), ?1 ~ [?0], ?1 ~ [?0], Entail(Eq, ?1)]

[Evidence(Eq, ?0), Entail(Eq, [?0])]

 */

pub struct Solve<'context> {
    pub(crate) context: &'context mut Context,
    pub(crate) unification_solved: HashMap<usize, TypeIdx>,
    pub(crate) unification_deferred: Vec<(usize, usize)>,
    pub(crate) unification_errors: Vec<UnifyError>,
    pub(crate) entailment_evidences: HashMap<u64, Assertion>,
    pub(crate) entailment_unifications_in: HashMap<usize, Vec<u64>>,
}

impl<'context> Solve<'context> {
    pub fn new(context: &'context mut Context) -> Self {
        Self {
            context,
            unification_solved: HashMap::new(),
            unification_deferred: Vec::new(),
            unification_errors: Vec::new(),
            entailment_evidences: HashMap::new(),
            entailment_unifications_in: HashMap::new(),
        }
    }

    pub fn as_unify<'solve>(&'solve mut self) -> Unify<'solve> {
        Unify::new(self.context)
    }

    pub(crate) fn step(&mut self) {
        while let Ok(constraint) = self.context.constraints.pop() {
            match constraint {
                Constraint::ClassEntail(_) => todo!(),
                Constraint::ClassEvidence(assertion) => {
                    let evidence = hash_assertion(&self.context.volatile.type_arena, &assertion);
                    for argument in &assertion.arguments {
                        if let Type::Unification { name } =
                            &self.context.volatile.type_arena[*argument]
                        {
                            self.entailment_unifications_in
                                .entry(*name)
                                .or_insert_with(|| vec![])
                                .push(evidence);
                        }
                    }
                    self.entailment_evidences.insert(evidence, assertion);
                }
                Constraint::UnifyDeep(t_name, u_name) => {
                    let t_idx = self.unification_solved.get(&t_name).copied();
                    let u_idx = self.unification_solved.get(&u_name).copied();
                    match (t_idx, u_idx) {
                        (Some(t_idx), Some(u_idx)) => {
                            self.as_unify().unify(t_idx, u_idx);
                        }
                        (None, Some(u_idx)) => {
                            self.unification_solved.insert(t_name, u_idx);
                            if let Some(evidences) = self.entailment_unifications_in.get(&t_name) {
                                for evidence in evidences {
                                    if let Some(mut assertion) =
                                        self.entailment_evidences.get(evidence).cloned()
                                    {
                                        for argument in &mut assertion.arguments {
                                            if let Type::Unification { name } =
                                                &self.context.volatile.type_arena[*argument]
                                            {
                                                if t_name == *name {
                                                    *argument = u_idx;
                                                }
                                            }
                                        }
                                        let evidence = hash_assertion(
                                            &self.context.volatile.type_arena,
                                            &assertion,
                                        );
                                        self.entailment_evidences.insert(evidence, assertion);
                                    }
                                }
                            }
                        }
                        (Some(t_idx), None) => {
                            self.unification_solved.insert(u_name, t_idx);
                            if let Some(evidences) = self.entailment_unifications_in.get(&u_name) {
                                for evidence in evidences {
                                    if let Some(mut assertion) =
                                        self.entailment_evidences.get(evidence).cloned()
                                    {
                                        for argument in &mut assertion.arguments {
                                            if let Type::Unification { name } =
                                                &self.context.volatile.type_arena[*argument]
                                            {
                                                if u_name == *name {
                                                    *argument = t_idx;
                                                }
                                            }
                                        }
                                        let evidence = hash_assertion(
                                            &self.context.volatile.type_arena,
                                            &assertion,
                                        );
                                        self.entailment_evidences.insert(evidence, assertion);
                                    }
                                }
                            }
                        }
                        (None, None) => {
                            // Avoids infinite loops with unsolvable unifications.
                            self.unification_deferred.push((t_name, u_name));
                        }
                    }
                }
                Constraint::UnifySolve(t_name, u_ty) => {
                    self.unification_solved.insert(t_name, u_ty);
                    if let Some(evidences) = self.entailment_unifications_in.get(&t_name) {
                        for evidence in evidences {
                            if let Some(mut assertion) =
                                self.entailment_evidences.get(evidence).cloned()
                            {
                                for argument in &mut assertion.arguments {
                                    if let Type::Unification { name } =
                                        &self.context.volatile.type_arena[*argument]
                                    {
                                        if t_name == *name {
                                            *argument = u_ty;
                                        }
                                    }
                                }
                                let evidence =
                                    hash_assertion(&self.context.volatile.type_arena, &assertion);
                                self.entailment_evidences.insert(evidence, assertion);
                            }
                        }
                    }
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
