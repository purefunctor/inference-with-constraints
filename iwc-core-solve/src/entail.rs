use std::{collections::HashMap, iter::zip};

use anyhow::bail;
use iwc_core_ast::ty::{hash::hash_assertion, Assertion, Type, TypeIdx};
use smol_str::SmolStr;

impl super::Solve {
    fn match_head(
        &mut self,
        substitutions: &mut HashMap<SmolStr, TypeIdx>,
        t_idx: TypeIdx,
        u_idx: TypeIdx,
    ) -> anyhow::Result<()> {
        match (
            &self.infer.volatile.type_arena[t_idx],
            &self.infer.volatile.type_arena[u_idx],
        ) {
            (Type::Constructor { name: t_name }, Type::Constructor { name: u_name })
                if t_name == u_name => {}
            (
                Type::Variable {
                    name: t_name,
                    rank: t_rank,
                },
                Type::Variable {
                    name: u_name,
                    rank: u_rank,
                },
            ) if (t_name, t_rank) == (u_name, u_rank) => {}
            (Type::Unification { name: t_name }, Type::Unification { name: u_name })
                if t_name == u_name => {}
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
                let t_result = *t_result;

                let u_arguments = u_arguments.clone();
                let u_result = *u_result;

                for (t_argument, u_argument) in zip(t_arguments, u_arguments) {
                    self.match_head(substitutions, t_argument, u_argument)?;
                }

                self.match_head(substitutions, t_result, u_result)?;
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

                self.match_head(substitutions, t_function, u_function)?;
                self.match_head(substitutions, t_argument, u_argument)?;
            }
            (Type::Variable { name, .. }, _) => match substitutions.get(name) {
                Some(v_idx) => {
                    self.match_head(substitutions, u_idx, *v_idx)?;
                }
                None => {
                    substitutions.insert(name.clone(), u_idx);
                }
            },
            (_, _) => bail!("Could not match instances..."),
        }

        Ok(())
    }

    pub fn entail(&mut self, assertion: Assertion) {
        let instances = self
            .infer
            .environment
            .lookup_instance(&assertion.name)
            .unwrap()
            .clone();

        for instance in instances {
            let mut substitutions = HashMap::new();

            let instance_arguments = &instance.assertion.arguments;
            let assertion_arguments = &assertion.arguments;

            for (instance_argument, assertion_argument) in
                zip(instance_arguments, assertion_arguments)
            {
                let instance_argument = *instance_argument;
                let assertion_argument = *assertion_argument;

                self.match_head(&mut substitutions, instance_argument, assertion_argument)
                    .unwrap();
            }

            for dependency in instance.dependencies {
                self.entail_dependency(&substitutions, dependency);
            }
        }
    }

    fn entail_dependency(
        &mut self,
        substitutions: &HashMap<SmolStr, TypeIdx>,
        mut assertion: Assertion,
    ) {
        for argument in &mut assertion.arguments {
            if let Type::Variable { name, .. } = &self.infer.volatile.type_arena[*argument] {
                if let Some(substitution) = substitutions.get(name) {
                    *argument = *substitution
                }
            }
        }

        let assertion_hash = hash_assertion(&self.infer.volatile.type_arena, &assertion);
        if self.entailment_evidences.contains(&assertion_hash) {
            println!("entail_helper: I am solved.");
            return;
        }
    }
}
