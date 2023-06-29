pub mod context;

#[cfg(test)]
mod tests {
    use iwc_core_ast::{expr::Expr, ty::Type};

    use crate::context::Context;

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
        dbg!(&context.volatile.type_arena[t]);
    }
}
