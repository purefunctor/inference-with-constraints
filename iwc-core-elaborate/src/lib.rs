pub mod context;
pub mod infer;
pub mod instantiate;
pub mod solve;
pub mod unify;

#[cfg(test)]
mod tests {
    use iwc_core_ast::ty::Type;
    use iwc_core_constraint::Constraint;

    use crate::{context::Context, solve::Solve};

    #[test]
    fn api_construction() {
        let ref mut context = Context::default();

        let int = context
            .volatile
            .type_arena
            .allocate(Type::Constructor { name: "Int".into() });

        context
            .constraints
            .push(Constraint::UnifyDeep(0, 1))
            .unwrap();
        context
            .constraints
            .push(Constraint::UnifySolve(1, int))
            .unwrap();

        let mut solve = Solve::new(context);

        solve.step();

        dbg!(&solve.unification_solved);
        dbg!(&solve.unification_errors);

        solve.step();

        dbg!(&solve.unification_solved);
        dbg!(&solve.unification_errors);
    }
}
