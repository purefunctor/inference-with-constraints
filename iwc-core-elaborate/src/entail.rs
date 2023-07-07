use std::{collections::HashMap, iter::zip};

use anyhow::Context;
use iwc_core_ast::ty::{Assertion, Type, TypeIdx};
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
            _ => false,
        }
    }

    pub fn entail(&mut self, assertion: Assertion) -> anyhow::Result<()> {
        let instances = self
            .context
            .environment
            .instances
            .get(&assertion.name)
            .cloned()
            .context(format!("No instances for {:?}", &assertion.name))?;

        for instance in instances {
            let mut substitutions = HashMap::new();

            let instance_arguments = &instance.assertion.arguments;
            let assertion_arguments = &assertion.arguments;

            let arguments_match = zip(instance_arguments, assertion_arguments).all(
                |(instance_argument, assertion_argument)| {
                    self.match_argument(&mut substitutions, *instance_argument, *assertion_argument)
                },
            );

            if arguments_match {
                break;
            }
        }

        Ok(())
    }
}

// Entailment may require dependencies to be solved too
//
// Entailment of an instance may require entailment of further dependencies.
//
// We can either defer them out into the constraint solver loop by emitting
// more `Entail` constraints, or solve them here as usual.
//
// The problem with deferring them is that dependencies require an environment
// to be passed. In particular, the substitution map for type variables that
// appear in instance heads.
//
// On the other hand, doing everything locally requires some degree of recursion
// which should be fine for simple cases, but would require some safeguards in
// case of recursive cases.


// struct EntailOne<'context> {
//     context: &'context mut crate::context::Context,
//     substitutions: HashMap<SmolStr, TypeIdx>,
// }

// impl<'context> EntailOne<'context> {
//     pub fn new(context: &'context mut crate::context::Context) -> Self {
//         Self { context, substitutions: HashMap::new() }
//     }

//     fn entail_one(&mut self, instance: , assertion: Assertion)
// }
