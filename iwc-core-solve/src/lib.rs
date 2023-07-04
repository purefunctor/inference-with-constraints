use std::{
    collections::{HashMap, HashSet},
    iter::zip,
};

use iwc_arena::Arena;
use iwc_core_ast::ty::{
    assertion_hash,
    pretty::{pretty_print_assertions, pretty_print_ty},
    Assertion, Type, TypeIdx,
};
use iwc_core_constraint::Constraint;
use iwc_core_error::UnifyError;
use iwc_core_infer::Infer;
use smol_str::SmolStr;

pub struct Solve {
    infer: Infer,
    unification_solved: HashMap<usize, TypeIdx>,
    unification_unsolved: Vec<(usize, usize)>,
    unification_errors: Vec<UnifyError>,
    entailment_evidences: HashSet<u64>,
}

impl Solve {
    pub fn new(infer: Infer) -> Self {
        Self {
            infer,
            unification_solved: HashMap::new(),
            unification_unsolved: Vec::new(),
            unification_errors: Vec::new(),
            entailment_evidences: HashSet::new(),
        }
    }
}

impl Solve {
    fn type_arena(&mut self) -> &Arena<Type> {
        &self.infer.volatile.type_arena
    }

    fn type_arena_mut(&mut self) -> &mut Arena<Type> {
        &mut self.infer.volatile.type_arena
    }

    fn entail(&mut self, assertion: Assertion) {
        #[derive(Debug)]
        struct Instance {
            assertion: Assertion,
            dependencies: Vec<Assertion>,
        }

        let a = self.type_arena_mut().allocate(Type::Variable {
            name: "a".into(),
            rank: 0,
        });
        let array = self.type_arena_mut().allocate(Type::Constructor {
            name: "Array".into(),
        });
        let array_a = self.type_arena_mut().allocate(Type::Application {
            function: array,
            argument: a,
        });

        let instances: HashMap<SmolStr, Vec<Instance>> = [(
            "Eq".into(),
            vec![Instance {
                assertion: Assertion {
                    name: "Eq".into(),
                    arguments: vec![array_a],
                },
                dependencies: vec![Assertion {
                    name: "Eq".into(),
                    arguments: vec![a],
                }],
            }],
        )]
        .into_iter()
        .collect();

        let instances = instances.get(&assertion.name).unwrap();

        for instance in instances {
            let instance_arguments = &instance.assertion.arguments;
            let assertion_arguments = &assertion.arguments;

            for (i, a) in zip(instance_arguments, assertion_arguments) {
                let i = *i;
                let a = *a;
                println!(
                    "{} = {}",
                    pretty_print_ty(&self.infer.volatile.type_arena, i),
                    pretty_print_ty(&self.infer.volatile.type_arena, a)
                );
            }
        }
    }

    pub(crate) fn step(&mut self, constraints: Vec<Constraint>) -> Vec<Constraint> {
        for constraint in constraints {
            match constraint {
                Constraint::ClassInfer(assertion) => self.entail(assertion),
                Constraint::ClassCheck(assertion) => {
                    self.entailment_evidences
                        .insert(assertion_hash(&self.infer.volatile.type_arena, &assertion));
                }
                Constraint::UnifyDeep(u_name, t_name) => {
                    let u_ty = self.unification_solved.get(&u_name);
                    let t_ty = self.unification_solved.get(&t_name);
                    match (u_ty, t_ty) {
                        (Some(u_ty), Some(t_ty)) => {
                            self.infer.unify(*u_ty, *t_ty);
                        }
                        (None, Some(t_ty)) => {
                            self.unification_solved.insert(u_name, *t_ty);
                        }
                        (Some(u_ty), None) => {
                            self.unification_solved.insert(t_name, *u_ty);
                        }
                        (None, None) => {
                            self.unification_unsolved.push((u_name, t_name));
                        }
                    }
                }
                Constraint::UnifySolve(name, ty) => {
                    self.unification_solved.insert(name, ty);
                }
                Constraint::UnifyError(error) => {
                    self.unification_errors.push(error);
                }
            }
        }

        let mut constraints = self.infer.take_constraints();

        self.unification_unsolved.retain(|(u_name, t_name)| {
            let u_ty = self.unification_solved.get(u_name);
            let t_ty = self.unification_solved.get(t_name);
            if u_ty.is_some() || t_ty.is_some() {
                constraints.push(Constraint::UnifyDeep(*u_name, *t_name));
                false
            } else {
                true
            }
        });

        constraints
    }

    pub fn solve(&mut self) {
        let mut constraints = self.infer.take_constraints();
        loop {
            constraints = self.step(constraints);
            if constraints.is_empty() {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use iwc_core_ast::{
        expr::Expr,
        ty::{pretty::pretty_print_ty, Assertion, Type, TypeVariableBinder},
    };
    use iwc_core_constraint::Constraint;
    use iwc_core_infer::Infer;

    use crate::Solve;

    fn default_infer() -> Infer {
        let mut context = Infer::default();

        let unit_ty = context.volatile.type_arena.allocate(Type::Constructor {
            name: "Unit".into(),
        });
        context.environment.insert_value_binding("unit", unit_ty);

        let int_ty = context
            .volatile
            .type_arena
            .allocate(Type::Constructor { name: "Int".into() });
        context.environment.insert_value_binding("zero", int_ty);

        let identity_ty = {
            let a_ty = context.volatile.type_arena.allocate(Type::Variable {
                name: "a".into(),
                rank: 0,
            });
            let a_to_a_ty = context.volatile.type_arena.allocate(Type::Function {
                arguments: vec![a_ty],
                result: a_ty,
            });
            context.volatile.type_arena.allocate(Type::Forall {
                variables: vec![TypeVariableBinder { name: "a".into() }],
                rank: 0,
                ty: a_to_a_ty,
            })
        };
        context
            .environment
            .insert_value_binding("identity", identity_ty);

        context
    }

    #[test]
    fn unification_solving() {
        let mut infer = default_infer();

        let identity_zero = {
            let identity = infer.volatile.expr_arena.allocate(Expr::Variable {
                name: "identity".into(),
            });
            let zero = infer.volatile.expr_arena.allocate(Expr::Variable {
                name: "zero".into(),
            });
            infer.volatile.expr_arena.allocate(Expr::Application {
                function: identity,
                arguments: vec![zero],
            })
        };

        let ty = infer.infer(identity_zero).unwrap();
        println!("{}", pretty_print_ty(&infer.volatile.type_arena, ty));

        let mut solver = Solve::new(infer);

        let mut constraints = solver.infer.take_constraints();

        constraints.reverse();

        dbg!(&constraints);
        dbg!(&solver.unification_solved);
        dbg!(&solver.unification_unsolved);
        dbg!(&solver.unification_errors);
        println!();

        constraints = solver.step(constraints);
        dbg!(&constraints);
        dbg!(&solver.unification_solved);
        dbg!(&solver.unification_unsolved);
        dbg!(&solver.unification_errors);
        println!();

        constraints = solver.step(constraints);
        dbg!(&constraints);
        dbg!(&solver.unification_solved);
        dbg!(&solver.unification_unsolved);
        dbg!(&solver.unification_errors);
        println!();
    }

    #[test]
    fn wanted_constraints() {
        let infer = Infer::default();
        let mut solve = Solve::new(infer);

        let array = solve.infer.volatile.type_arena.allocate(Type::Constructor {
            name: "Array".into(),
        });
        let u_zero = solve.infer.volatile.fresh_unification();
        let u_one = solve.infer.volatile.fresh_unification();
        let array_zero = solve.infer.volatile.type_arena.allocate(Type::Application {
            function: array,
            argument: u_zero,
        });

        let mut constraints = vec![
            Constraint::ClassCheck(Assertion {
                name: "Eq".into(),
                arguments: vec![u_one],
            }),
            Constraint::UnifySolve(1, array_zero),
            Constraint::UnifySolve(1, array_zero),
            Constraint::ClassInfer(Assertion {
                name: "Eq".into(),
                arguments: vec![array_zero],
            }),
        ];

        constraints = solve.step(constraints);

        dbg!(constraints);
    }
}
