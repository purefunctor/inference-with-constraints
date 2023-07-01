pub mod context;
pub mod solver;

#[cfg(test)]
mod tests {
    use iwc_core_ast::{
        expr::Expr,
        ty::{pretty::pretty_print_ty, Type, TypeVariableBinder},
    };

    use crate::context::Context;

    fn default_context() -> Context {
        let mut context = Context::default();

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
        let mut context = default_context();

        let identity_zero = {
            let identity = context.volatile.expr_arena.allocate(Expr::Variable {
                name: "identity".into(),
            });
            let zero = context.volatile.expr_arena.allocate(Expr::Variable {
                name: "zero".into(),
            });
            context.volatile.expr_arena.allocate(Expr::Application {
                function: identity,
                arguments: vec![zero],
            })
        };

        let ty = context.infer(identity_zero).unwrap();
        println!("{}", pretty_print_ty(&context.volatile.type_arena, ty));

        let mut solver = context.solver();

        let mut constraints = solver.take_constraints();

        constraints.reverse();

        dbg!(&constraints);
        dbg!(&solver.unification_solved);
        dbg!(&solver.unification_unsolved);
        println!();

        constraints = solver.step(constraints).unwrap();
        dbg!(&constraints);
        dbg!(&solver.unification_solved);
        dbg!(&solver.unification_unsolved);
        println!();

        constraints = solver.step(constraints).unwrap();
        dbg!(&constraints);
        dbg!(&solver.unification_solved);
        dbg!(&solver.unification_unsolved);
        println!();
    }
}
