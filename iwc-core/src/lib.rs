pub mod context;
pub mod types;

#[cfg(test)]
mod tests {
    use crate::context::Context;

    #[test]
    pub fn lambda_inference() {
        let mut context = Context::default();

        let identity_type = {
            let a = context.ty_variable("a", 0);
            let a_to_a = context.ty_function(a, a);
            context.ty_forall(&["a"], 0, a_to_a)
        };

        context.bind_type("identity", identity_type);

        let identity_expr = context.expr_variable("identity");
        let impredicative = context.expr_application(identity_expr, identity_expr);

        match context.infer(impredicative) {
            Ok(t) => println!("{:?}", &context.ty_arena[t]),
            Err(e) => println!("Failed: {:?}", e),
        }
    }
}
