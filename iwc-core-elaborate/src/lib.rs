pub mod context;
pub mod entail;
pub mod infer;
pub mod instantiate;
pub mod solve;
pub mod unify;

#[cfg(test)]
mod tests {
    use iwc_core_ast::ty::{
        pretty::{pretty_print_assertions, pretty_print_ty},
        Assertion, Class, FunctionalDependency, Instance, Type,
    };
    use iwc_core_constraint::Constraint;

    use crate::{context::Context, solve::Solve};

    #[test]
    fn entailment_concrete() {
        let context = &mut Context::default();

        let int = context
            .volatile
            .type_arena
            .allocate(Type::Constructor { name: "Int".into() });

        context.environment.classes.insert(
            "Eq".into(),
            Class {
                functional_dependencies: vec![],
            },
        );

        context.environment.instances.insert(
            "Eq".into(),
            vec![Instance {
                assertion: Assertion {
                    name: "Eq".into(),
                    arguments: vec![int],
                },
                dependencies: vec![],
            }],
        );

        let index = context.fresh_index();
        context
            .constraints
            .push(Constraint::ClassEntail(
                index,
                Assertion {
                    name: "Eq".into(),
                    arguments: vec![int],
                },
            ))
            .unwrap();

        let mut solve = Solve::new(context);

        solve.step();

        dbg!(&solve.entailment_evidences);
    }

    #[test]
    fn entailment_unification_deferred() {
        let context = &mut Context::default();

        let int = context
            .volatile
            .type_arena
            .allocate(Type::Constructor { name: "Int".into() });

        let u_one_name = context.fresh_index();
        let u_one = context
            .volatile
            .type_arena
            .allocate(Type::Unification { name: u_one_name });

        context.environment.instances.insert(
            "Eq".into(),
            vec![Instance {
                assertion: Assertion {
                    name: "Eq".into(),
                    arguments: vec![int],
                },
                dependencies: vec![],
            }],
        );

        context.environment.classes.insert(
            "Eq".into(),
            Class {
                functional_dependencies: vec![],
            },
        );

        let index = context.fresh_index();

        context
            .constraints
            .push(Constraint::ClassEntail(
                index,
                Assertion {
                    name: "Eq".into(),
                    arguments: vec![u_one],
                },
            ))
            .unwrap();

        context
            .constraints
            .push(Constraint::UnifySolve(u_one_name, int))
            .unwrap();

        let mut solve = Solve::new(context);

        solve.step();

        dbg!(&solve.unification_solved);
        dbg!(&solve.entailment_evidences);

        solve.step();

        dbg!(&solve.unification_solved);
        dbg!(&solve.entailment_evidences);
    }

    #[test]
    fn entailment_dependencies() {
        let context = &mut Context::default();

        context.environment.classes.insert(
            "Eq".into(),
            Class {
                functional_dependencies: vec![],
            },
        );

        let array = context.volatile.type_arena.allocate(Type::Constructor {
            name: "Array".into(),
        });

        let int = context
            .volatile
            .type_arena
            .allocate(Type::Constructor { name: "Int".into() });

        let a = context.volatile.type_arena.allocate(Type::Variable {
            name: "a".into(),
            rank: 0,
        });

        let array_a = context.volatile.type_arena.allocate(Type::Application {
            function: array,
            arguments: vec![a],
        });

        context.environment.instances.insert(
            "Eq".into(),
            vec![
                Instance {
                    assertion: Assertion {
                        name: "Eq".into(),
                        arguments: vec![array_a],
                    },
                    dependencies: vec![Assertion {
                        name: "Eq".into(),
                        arguments: vec![a],
                    }],
                },
                Instance {
                    assertion: Assertion {
                        name: "Eq".into(),
                        arguments: vec![int],
                    },
                    dependencies: vec![],
                },
            ],
        );

        let array_int = context.volatile.type_arena.allocate(Type::Application {
            function: array,
            arguments: vec![int],
        });

        let index = context.fresh_index();
        context
            .constraints
            .push(Constraint::ClassEntail(
                index,
                Assertion {
                    name: "Eq".into(),
                    arguments: vec![array_int],
                },
            ))
            .unwrap();

        let mut solve = Solve::new(context);

        solve.step();

        dbg!(&solve.entailment_evidences);
    }

    #[test]
    fn entailment_recursion() {
        let mut context = Context::default();

        context.environment.classes.insert(
            "Append".into(),
            Class {
                functional_dependencies: vec![FunctionalDependency {
                    domain: vec![0, 1],
                    codomain: vec![2],
                }],
            },
        );

        let nil = context
            .volatile
            .type_arena
            .allocate(Type::Constructor { name: "Nil".into() });

        let cons = context.volatile.type_arena.allocate(Type::Constructor {
            name: "Cons".into(),
        });

        let zero = context.volatile.type_arena.allocate(Type::Constructor {
            name: "Zero".into(),
        });

        let one = context
            .volatile
            .type_arena
            .allocate(Type::Constructor { name: "One".into() });

        {
            let x = context.volatile.type_arena.allocate(Type::Variable {
                name: "x".into(),
                rank: 0,
            });
            let xs = context.volatile.type_arena.allocate(Type::Variable {
                name: "xs".into(),
                rank: 0,
            });
            let ys = context.volatile.type_arena.allocate(Type::Variable {
                name: "ys".into(),
                rank: 0,
            });
            let zs = context.volatile.type_arena.allocate(Type::Variable {
                name: "zs".into(),
                rank: 0,
            });
            let cons_x_xs = context.volatile.type_arena.allocate(Type::Application {
                function: cons,
                arguments: vec![x, xs],
            });
            let cons_x_zs = context.volatile.type_arena.allocate(Type::Application {
                function: cons,
                arguments: vec![x, zs],
            });
            context.environment.instances.insert(
                "Append".into(),
                vec![
                    Instance {
                        assertion: Assertion {
                            name: "Append".into(),
                            arguments: vec![nil, ys, ys],
                        },
                        dependencies: vec![],
                    },
                    Instance {
                        assertion: Assertion {
                            name: "Append".into(),
                            arguments: vec![cons_x_xs, ys, cons_x_zs],
                        },
                        dependencies: vec![Assertion {
                            name: "Append".into(),
                            arguments: vec![xs, ys, zs],
                        }],
                    },
                ],
            );
        }

        let u = context.fresh_unification();

        let cons_0_nil = context.volatile.type_arena.allocate(Type::Application {
            function: cons,
            arguments: vec![zero, nil],
        });

        let cons_1_nil = context.volatile.type_arena.allocate(Type::Application {
            function: cons,
            arguments: vec![one, nil],
        });

        let index = context.fresh_index();
        context
            .constraints
            .push(Constraint::ClassEntail(
                index,
                Assertion {
                    name: "Append".into(),
                    arguments: vec![cons_0_nil, cons_1_nil, u],
                },
            ))
            .unwrap();

        let mut solve = Solve::new(&mut context);

        solve.step();

        dbg!(&solve.entailment_evidences);

        println!(
            "{}",
            pretty_print_assertions(
                &solve.context.volatile.type_arena,
                &[Assertion {
                    name: "Append".into(),
                    arguments: vec![cons_0_nil, cons_1_nil, u],
                }]
            )
        );

        for (u, t_idx) in &solve.unification_solved {
            println!(
                "?{} ~ {}",
                u,
                pretty_print_ty(&solve.context.volatile.type_arena, *t_idx)
            );
        }
    }
}
