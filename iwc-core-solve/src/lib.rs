pub mod entail;
pub mod solve;

use std::collections::{HashMap, HashSet};

use iwc_core_ast::ty::TypeIdx;
use iwc_core_error::UnifyError;
use iwc_core_infer::Infer;

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

#[cfg(test)]
mod tests {
    use iwc_core_ast::{
        expr::Expr,
        ty::{pretty::pretty_print_ty, Assertion, Instance, Type, TypeVariableBinder},
    };
    use iwc_core_constraint::Constraint;
    use iwc_core_infer::Infer;

    use crate::Solve;

    fn default_infer() -> Infer {
        let mut context = Infer::default();

        let unit_ty = context.volatile.type_arena.allocate(Type::Constructor {
            name: "Unit".into(),
        });
        context.environment.insert_value("unit", unit_ty);

        let int_ty = context
            .volatile
            .type_arena
            .allocate(Type::Constructor { name: "Int".into() });
        context.environment.insert_value("zero", int_ty);

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
        context.environment.insert_value("identity", identity_ty);

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
        let v_a = solve.infer.volatile.type_arena.allocate(Type::Variable {
            name: "a".into(),
            rank: 0,
        });
        let array_a = solve.infer.volatile.type_arena.allocate(Type::Application {
            function: array,
            argument: v_a,
        });

        solve.infer.environment.insert_instance(
            "Eq",
            Instance {
                assertion: Assertion {
                    name: "Eq".into(),
                    arguments: vec![array_a],
                },
                dependencies: vec![Assertion {
                    name: "Eq".into(),
                    arguments: vec![v_a],
                }],
            },
        );

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
