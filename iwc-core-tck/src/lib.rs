pub mod context;

#[cfg(test)]
mod tests {
    use iwc_core_ast::{
        expr::Expr,
        ty::{
            pretty::{pretty_print_assertions, pretty_print_ty},
            Assertion,
        },
        ty::{Type, TypeVariableBinder},
    };

    use crate::context::{Constraint, Context};

    #[test]
    pub fn function_unification() {
        let mut context = Context::default();

        let unit = context.volatile.type_arena.allocate(Type::Constructor {
            name: "Unit".into(),
        });
        let f = context.volatile.type_arena.allocate(Type::Function {
            argument: unit,
            result: unit,
        });

        let u_zero = context
            .volatile
            .type_arena
            .allocate(Type::Unification { name: 0 });
        let u_one = context
            .volatile
            .type_arena
            .allocate(Type::Unification { name: 1 });
        let g = context.volatile.type_arena.allocate(Type::Function {
            argument: u_zero,
            result: u_one,
        });

        context.unify(f, g).unwrap();
        dbg!(context.volatile.constraints);
    }

    #[test]
    pub fn lambda_inference() {
        let mut context = Context::default();

        let b = context
            .volatile
            .expr_arena
            .allocate(Expr::Variable { name: "a".into() });

        let f = context.volatile.expr_arena.allocate(Expr::Lambda {
            arguments: vec!["a".into(), "b".into()],
            body: b,
        });

        let t = context.infer(f).unwrap();
        println!("{}", pretty_print_ty(&context.volatile.type_arena, t))
    }

    #[test]
    pub fn polytype_inference() {
        let mut context = Context::default();

        let eq = {
            let boolean = context.volatile.type_arena.allocate(Type::Constructor {
                name: "Boolean".into(),
            });
            let a = context.volatile.type_arena.allocate(Type::Variable {
                name: "a".into(),
                rank: 0,
            });
            let a_boolean = context.volatile.type_arena.allocate(Type::Function {
                argument: a,
                result: boolean,
            });
            let a_a_boolean = context.volatile.type_arena.allocate(Type::Function {
                argument: a,
                result: a_boolean,
            });
            let eq_a_a_boolean = context.volatile.type_arena.allocate(Type::Constrained {
                assertions: vec![
                    Assertion {
                        name: "Eq".into(),
                        arguments: vec![a],
                    },
                    Assertion {
                        name: "Ord".into(),
                        arguments: vec![a],
                    },
                ],
                ty: a_a_boolean,
            });
            context.volatile.type_arena.allocate(Type::Forall {
                variables: vec![TypeVariableBinder { name: "a".into() }],
                rank: 0,
                ty: eq_a_a_boolean,
            })
        };

        context.environment.insert_value_binding("eq", eq);

        let eq = context
            .volatile
            .expr_arena
            .allocate(Expr::Variable { name: "eq".into() });

        let ty = context.infer(eq).unwrap();
        let ty = context.instantiate(ty);

        println!("{}", pretty_print_ty(&context.volatile.type_arena, ty));

        let mut assertions = vec![];
        for constraint in context.volatile.constraints {
            if let Constraint::ClassAssertion(assertion) = constraint {
                assertions.push(assertion);
            }
        }

        println!(
            "{}",
            pretty_print_assertions(&context.volatile.type_arena, &assertions)
        );
    }
}
