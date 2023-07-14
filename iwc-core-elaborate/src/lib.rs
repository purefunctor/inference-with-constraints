pub mod context;
pub mod entail;
pub mod infer;
pub mod instantiate;
pub mod solve;
pub mod unify;

#[cfg(test)]
mod tests {
    // #[test]
    // fn entailment_implementation() {
    //     let ref mut context = Context::default();

    //     let int = context
    //         .volatile
    //         .type_arena
    //         .allocate(Type::Constructor { name: "Int".into() });

    //     let a = context.volatile.type_arena.allocate(Type::Variable {
    //         name: "a".into(),
    //         rank: 0,
    //     });

    //     let array = context.volatile.type_arena.allocate(Type::Constructor {
    //         name: "Array".into(),
    //     });

    //     let array_a = context.volatile.type_arena.allocate(Type::Application {
    //         function: array,
    //         argument: a,
    //     });

    //     let array_int = context.volatile.type_arena.allocate(Type::Application {
    //         function: array,
    //         argument: int,
    //     });

    //     context.environment.instances.insert(
    //         "Eq".into(),
    //         vec![
    //             Instance {
    //                 assertion: Assertion {
    //                     name: "Eq".into(),
    //                     arguments: vec![int],
    //                 },
    //                 dependencies: vec![],
    //             },
    //             Instance {
    //                 assertion: Assertion {
    //                     name: "Eq".into(),
    //                     arguments: vec![array_a],
    //                 },
    //                 dependencies: vec![Assertion {
    //                     name: "Eq".into(),
    //                     arguments: vec![a],
    //                 }],
    //             },
    //         ],
    //     );

    //     // entail([Int])
    //     context
    //         .constraints
    //         .push(Constraint::ClassEntail(Assertion {
    //             name: "Eq".into(),
    //             arguments: vec![int],
    //         }))
    //         .unwrap();

    //     let mut entail = Entail::new(context);

    //     let assertion = Assertion {
    //         name: "Eq".into(),
    //         arguments: vec![array_int],
    //     };

    //     entail.entail(&assertion);
    // }
}
