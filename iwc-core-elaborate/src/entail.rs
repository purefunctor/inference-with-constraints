use std::{
    collections::{HashMap, HashSet},
    iter::zip,
};

use iwc_core_ast::ty::{
    traversal::{default_traverse_ty, Traversal},
    Assertion, FunctionalDependency, Type, TypeIdx,
};
use smol_str::SmolStr;

pub struct Entail<'context> {
    context: &'context mut crate::context::Context,
}

#[derive(Debug)]
pub enum Evidence {
    Dictionary { dependencies: Vec<usize> },
}

#[derive(Debug)]
pub enum EntailResult {
    Solved {
        evidence: Evidence,
        instance_assertion: Assertion,
    },
    Depends {
        evidence: Evidence,
        instance_assertion: Assertion,
        instance_dependencies: Vec<(usize, Assertion)>,
    },
    Deferred {
        needs_solution: HashSet<(usize, usize)>,
    },
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
                    arguments: t_arguments,
                },
                Type::Application {
                    function: u_function,
                    arguments: u_arguments,
                },
            ) => {
                let t_function = *t_function;
                let t_arguments = t_arguments.clone();

                let u_function = *u_function;
                let u_arguments = u_arguments.clone();

                self.match_argument(substitutions, t_function, u_function)
                    && zip(t_arguments, u_arguments).all(|(t_argument, u_argument)| {
                        self.match_argument(substitutions, t_argument, u_argument)
                    })
            }
            (Type::Variable { name, .. }, _) => match substitutions.get(name) {
                Some(t_idx) => self.match_argument(substitutions, *t_idx, u_idx),
                None => {
                    substitutions.insert(name.clone(), u_idx);
                    true
                }
            },
            (_, Type::Unification { .. }) => true,
            _ => false,
        }
    }

    fn needs_solution(&self, assertion: &Assertion) -> HashSet<(usize, usize)> {
        let mut needs_solution = HashSet::new();

        let class = self
            .context
            .environment
            .classes
            .get(&assertion.name)
            .cloned()
            .unwrap();

        if class.functional_dependencies.is_empty() {
            for (index, argument) in assertion.arguments.iter().copied().enumerate() {
                if let Type::Unification { name } = &self.context.volatile.type_arena[argument] {
                    needs_solution.insert((index, *name));
                }
            }
        } else {
            for FunctionalDependency { domain, .. } in &class.functional_dependencies {
                for argument_index in domain {
                    let argument = assertion.arguments[*argument_index];
                    if let Type::Unification { name } = &self.context.volatile.type_arena[argument]
                    {
                        needs_solution.insert((*argument_index, *name));
                    }
                }
            }
        }

        needs_solution
    }

    pub fn entail(&mut self, assertion: &Assertion) -> EntailResult {
        let needs_solution = self.needs_solution(assertion);
        if !needs_solution.is_empty() {
            return EntailResult::Deferred { needs_solution };
        }

        let instances = self
            .context
            .environment
            .instances
            .get(&assertion.name)
            .cloned()
            .unwrap();

        for instance in instances {
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

            if !arguments_match {
                continue;
            }

            let mut sgf = SubstituteGeneralizingFree::new(self.context, &mut substitutions);

            if instance.dependencies.is_empty() {
                let instance_assertion = sgf.on_assertion(&instance.assertion);

                let needs_solution = self.needs_solution(assertion);
                if !needs_solution.is_empty() {
                    return EntailResult::Deferred { needs_solution };
                }

                let instance_evidence = Evidence::Dictionary {
                    dependencies: vec![],
                };

                return EntailResult::Solved {
                    evidence: instance_evidence,
                    instance_assertion,
                };
            } else {
                let instance_assertion = sgf.on_assertion(&instance.assertion);

                let mut instance_dependencies = vec![];
                let mut dictionary_dependencies = vec![];
                for dependency in &instance.dependencies {
                    let index = sgf.context.fresh_index();
                    let dependency = sgf.on_assertion(dependency);
                    instance_dependencies.push((index, dependency));
                    dictionary_dependencies.push(index);
                }

                let instance_evidence = Evidence::Dictionary {
                    dependencies: dictionary_dependencies,
                };

                return EntailResult::Depends {
                    evidence: instance_evidence,
                    instance_assertion,
                    instance_dependencies,
                };
            }
        }

        EntailResult::Deferred {
            needs_solution: HashSet::new(),
        }
    }
}

struct SubstituteGeneralizingFree<'context> {
    context: &'context mut crate::context::Context,
    substitutions: &'context mut HashMap<SmolStr, TypeIdx>,
}

impl<'context> SubstituteGeneralizingFree<'context> {
    fn new(
        context: &'context mut crate::context::Context,
        substitutions: &'context mut HashMap<SmolStr, TypeIdx>,
    ) -> Self {
        Self {
            context,
            substitutions,
        }
    }

    fn on_assertion(&mut self, assertion: &Assertion) -> Assertion {
        let name = assertion.name.clone();
        let mut arguments = assertion.arguments.clone();

        arguments.iter_mut().for_each(|argument| {
            *argument = self.traverse_ty(*argument);
        });

        Assertion { name, arguments }
    }
}

impl<'context> Traversal for SubstituteGeneralizingFree<'context> {
    fn arena(&mut self) -> &mut iwc_arena::Arena<Type> {
        &mut self.context.volatile.type_arena
    }

    fn traverse_ty(&mut self, ty_idx: TypeIdx) -> TypeIdx {
        if let Type::Variable { name, .. } = &self.context.volatile.type_arena[ty_idx] {
            *self
                .substitutions
                .entry(name.clone())
                .or_insert_with(|| self.context.fresh_unification())
        } else {
            default_traverse_ty(self, ty_idx)
        }
    }
}
