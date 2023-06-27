pub mod context;

#[cfg(test)]
mod tests {
    use iwc_core_ast::{
        expr::Expr,
        ty::{Assertion, Instance, Ty},
    };
    use tinyvec::tiny_vec;

    use crate::context::Context;

    #[test]
    fn entailment_test() {
        let mut context = Context::default();

        // class DoubleUp a b
        // instance DoubleUp Unit (Unit, Unit)
        // doubleUp :: forall a b. Const a b => a -> b

        let double_up_value = {
            let a = context.volatile.ty_arena.allocate(Ty::Variable {
                name: "a".into(),
                rank: 0,
            });
            let b = context.volatile.ty_arena.allocate(Ty::Variable {
                name: "b".into(),
                rank: 0,
            });
            let m = context.volatile.ty_arena.allocate(Ty::Function {
                argument: a,
                result: b,
            });
            let c = context.volatile.ty_arena.allocate(Ty::Constrained {
                assertions: tiny_vec!(Assertion {
                    name: "DoubleUp".into(),
                    arguments: tiny_vec!(a, b),
                }),
                ty: m,
            });
            context.volatile.ty_arena.allocate(Ty::Forall {
                variables: tiny_vec!("a".into(), "b".into()),
                rank: 0,
                ty: c,
            })
        };

        context
            .environment
            .bindings
            .insert("doubleUp".into(), double_up_value);

        let double_up_instance = {
            let u = context.volatile.ty_arena.allocate(Ty::Unit);
            let p = context
                .volatile
                .ty_arena
                .allocate(Ty::Pair { left: u, right: u });
            let a = Assertion {
                name: "DoubleUp".into(),
                arguments: tiny_vec!(u, p),
            };
            Instance {
                assertion: a,
                dependencies: tiny_vec!(),
            }
        };

        context
            .environment
            .instances
            .insert("DoubleUp".into(), vec![double_up_instance]);

        let expression = {
            let function_expr = context.volatile.expr_arena.allocate(Expr::Variable {
                name: "doubleUp".into(),
            });
            let argument_expr = context.volatile.expr_arena.allocate(Expr::Unit);
            context.volatile.expr_arena.allocate(Expr::Application {
                function: function_expr,
                argument: argument_expr,
            })
        };

        match context.infer(expression) {
            Ok(t_idx) => println!("{:?}", context.volatile.ty_arena[t_idx]),
            Err(e) => eprintln!("Failed with {:?}", e),
        }

        match context.solve() {
            Ok(_) => (),
            Err(e) => eprintln!("Failed with {:?}", e),
        }
    }
}
