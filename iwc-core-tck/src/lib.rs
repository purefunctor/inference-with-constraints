pub mod context;

#[cfg(test)]
mod tests {
    use iwc_core_ast::ty::{Type, TypeVariableBinder};

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
}
