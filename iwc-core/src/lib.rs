pub mod context;
pub mod types;

#[cfg(test)]
mod tests {
    use crate::context::Context;

    #[test]
    pub fn lambda_inference() {
        let mut context = Context::default();

        let identity = {
            let a = context.expr_variable("a");
            context.expr_lambda("a", a)
        };

        let unit = context.expr_unit();

        let identity_unit = context.expr_application(identity, unit);

        match context.infer(identity_unit) {
            Ok(t) => println!("Inferred: {:?}", &context.ty_arena[t]),
            Err(e) => println!("Failed: {:?}", e),
        }

        context.solve().unwrap();
    }
}
