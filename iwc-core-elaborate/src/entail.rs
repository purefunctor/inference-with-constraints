use std::{collections::HashMap, iter::zip};

use iwc_core_ast::ty::{Assertion, Instance, Type, TypeIdx};
use iwc_core_constraint::Constraint;
use smol_str::SmolStr;

pub struct Entail<'context> {
    context: &'context mut crate::context::Context,
}

#[derive(Debug)]
struct InstanceMatch {
    dependencies: Vec<Assertion>,
    substitutions: HashMap<SmolStr, TypeIdx>,
}

#[derive(Debug)]
pub enum Evidence {
    Dictionary(Vec<Evidence>),
    Variable(usize),
}

#[derive(Debug)]
pub enum EntailResult {
    Solved {
        evidence: Evidence,
    },
    Depends {
        evidence: Evidence,
        instance: Instance,
        substitution: HashMap<SmolStr, TypeIdx>,
    },
    Deferred,
}

impl<'context> Entail<'context> {
    pub fn new(context: &'context mut crate::context::Context) -> Self {
        Self { context }
    }

    fn match_argument(
        &mut self,
        substitutions: &mut HashMap<SmolStr, TypeIdx>,
        t_idx: TypeIdx,
        u_idx: TypeIdx,
    ) -> bool {
        match (
            &self.context.volatile.type_arena[t_idx],
            &self.context.volatile.type_arena[u_idx],
        ) {
            (Type::Constructor { name: t_name }, Type::Constructor { name: u_name }) => {
                t_name == u_name
            }
            (
                Type::Variable {
                    name: t_name,
                    rank: t_rank,
                },
                Type::Variable {
                    name: u_name,
                    rank: u_rank,
                },
            ) => (t_name, t_rank) == (u_name, u_rank),
            (Type::Unification { name: t_name }, Type::Unification { name: u_name }) => {
                t_name == u_name
            }
            (
                Type::Function {
                    arguments: t_arguments,
                    result: t_result,
                },
                Type::Function {
                    arguments: u_arguments,
                    result: u_result,
                },
            ) => {
                let t_arguments = t_arguments.clone();
                let u_arguments = u_arguments.clone();

                let t_result = *t_result;
                let u_result = *u_result;

                zip(t_arguments, u_arguments).all(|(t_argument, u_argument)| {
                    self.match_argument(substitutions, t_argument, u_argument)
                }) && self.match_argument(substitutions, t_result, u_result)
            }
            (
                Type::Application {
                    function: t_function,
                    argument: t_argument,
                },
                Type::Application {
                    function: u_function,
                    argument: u_argument,
                },
            ) => {
                let t_function = *t_function;
                let t_argument = *t_argument;

                let u_function = *u_function;
                let u_argument = *u_argument;

                self.match_argument(substitutions, t_function, u_function)
                    && self.match_argument(substitutions, t_argument, u_argument)
            }
            (Type::Variable { name, .. }, _) => match substitutions.get(name) {
                Some(t_idx) => self.match_argument(substitutions, *t_idx, u_idx),
                None => {
                    substitutions.insert(name.clone(), u_idx);
                    true
                }
            },
            _ => false,
        }
    }

    fn try_instance(
        &mut self,
        instance: &Instance,
        assertion: &Assertion,
    ) -> Option<InstanceMatch> {
        // TODO: reject if there exists unification variables in the assertion
        // TODO: this is also subject to when we implement functional dependencies

        let mut substitutions = HashMap::new();

        let instance_arguments = &instance.assertion.arguments;
        let assertion_arguments = &assertion.arguments;

        let arguments_match = zip(
            instance_arguments.iter().copied(),
            assertion_arguments.iter().copied(),
        )
        .all(|(instance_argument, assertion_argument)| {
            self.match_argument(&mut substitutions, instance_argument, assertion_argument)
        });

        if arguments_match {
            let dependencies = instance.dependencies.clone();
            Some(InstanceMatch {
                dependencies,
                substitutions,
            })
        } else {
            None
        }
    }

    pub fn entail(&mut self, assertion: &Assertion) -> EntailResult {
        let instances = self.context.environment.find_instances(&assertion.name);

        for instance in &instances {
            if let Some(result) = self.entail_with(&assertion, instance) {
                return result;
            }
        }

        EntailResult::Deferred
    }

    pub fn entail_with(
        &mut self,
        assertion: &Assertion,
        instance: &Instance,
    ) -> Option<EntailResult> {
        let InstanceMatch {
            dependencies,
            substitutions,
        } = self.try_instance(instance, assertion)?;

        let dependencies: Vec<_> = dependencies
            .into_iter()
            .map(|dependency| {
                let index = self.context.fresh_index();
                self.context
                    .constraints
                    .push(Constraint::ClassEntail(index, dependency))
                    .unwrap();
                Evidence::Variable(index)
            })
            .collect();

        if dependencies.is_empty() {
            Some(EntailResult::Solved {
                evidence: Evidence::Dictionary(dependencies),
            })
        } else {
            Some(EntailResult::Depends {
                evidence: Evidence::Dictionary(dependencies),
                instance: instance.clone(),
                substitution: substitutions,
            })
        }
    }
}
