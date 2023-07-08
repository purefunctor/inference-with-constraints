use std::{collections::HashMap, iter::zip};

use anyhow::Context;
use iwc_core_ast::ty::{pretty::pretty_print_ty, Assertion, Type, TypeIdx};
use smol_str::SmolStr;

pub struct Entail<'context> {
    context: &'context mut crate::context::Context,
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
        println!(
            "{:?} = {:?}",
            pretty_print_ty(&self.context.volatile.type_arena, t_idx),
            pretty_print_ty(&self.context.volatile.type_arena, u_idx)
        );

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
            (_, Type::Variable { name, .. }) => {
                let u_idx = *substitutions.get(name).unwrap();
                self.match_argument(substitutions, t_idx, u_idx)
            }
            _ => false,
        }
    }

    pub fn entail(&mut self, assertion: &Assertion) {
        self.entail_core(None, assertion);
    }

    fn entail_core(
        &mut self,
        mut substitutions: Option<&mut HashMap<SmolStr, TypeIdx>>,
        assertion: &Assertion,
    ) -> bool {
        let instances = self
            .context
            .environment
            .instances
            .get(&assertion.name)
            .cloned()
            .context(format!("No instance found! {}", assertion.name))
            .unwrap();

        for instance in instances {
            let mut local_substitutions = HashMap::new();

            let substitutions = match &mut substitutions {
                Some(substitutions) => substitutions,
                None => &mut local_substitutions,
            };

            let assertion_entails = zip(&instance.assertion.arguments, &assertion.arguments).all(
                |(instance_argument, assertion_argument)| {
                    self.match_argument(substitutions, *instance_argument, *assertion_argument)
                },
            );

            if !assertion_entails {
                continue;
            }

            let dependencies_entail = instance
                .dependencies
                .iter()
                .all(|dependency| self.entail_core(Some(substitutions), dependency));

            if dependencies_entail {
                return true;
            }
        }

        false
    }
}
