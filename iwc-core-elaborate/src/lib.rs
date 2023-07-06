pub mod context;
pub mod infer;
pub mod instantiate;
pub mod solve;
pub mod unify;

#[cfg(test)]
mod tests {
    use iwc_core_ast::ty::{Assertion, Type};
    use iwc_core_constraint::Constraint;

    use crate::{context::Context, solve::Solve};

    #[test]
    fn api_construction() {
        let ref mut context = Context::default();

        let int = context
            .volatile
            .type_arena
            .allocate(Type::Constructor { name: "Int".into() });

        let u_zero = context.fresh_unification();

        context
            .constraints
            .push(Constraint::ClassEvidence(Assertion {
                name: "Eq".into(),
                arguments: vec![u_zero],
            }))
            .unwrap();

        context
            .constraints
            .push(Constraint::UnifySolve(0, int))
            .unwrap();

        let mut solve = Solve::new(context);

        solve.step();

        dbg!(solve.entailment_evidences);
        dbg!(solve.entailment_unifications_in);
    }
}
