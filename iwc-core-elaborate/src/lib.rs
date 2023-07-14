pub mod context;
pub mod entail;
pub mod infer;
pub mod instantiate;
pub mod solve;
pub mod unify;

#[cfg(test)]
mod tests {
    use iwc_core_ast::ty::{Assertion, Instance, Type};
    use iwc_core_constraint::Constraint;

    use crate::{context::Context, solve::Solve};

    #[test]
    fn entailment_concrete() {
        let context = &mut Context::default();

        let int = context
            .volatile
            .type_arena
            .allocate(Type::Constructor { name: "Int".into() });

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

        dbg!(solve.entailment_evidence);
    }
}
