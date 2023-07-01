use iwc_arena::Arena;

use crate::expr::{Expr, ExprIdx};

pub trait Traversal: Sized {
    fn arena(&mut self) -> &mut Arena<Expr>;

    fn traverse_expr(&mut self, expr_idx: ExprIdx) -> ExprIdx {
        default_traverse_expr(self, expr_idx)
    }
}

pub fn default_traverse_expr<T: Traversal>(traversal: &mut T, expr_idx: ExprIdx) -> ExprIdx {
    match &traversal.arena()[expr_idx] {
        Expr::Constructor { .. } => expr_idx,
        Expr::Variable { .. } => expr_idx,
        Expr::Application {
            function,
            arguments,
        } => {
            let function = *function;
            let mut arguments = arguments.clone();

            let function = traversal.traverse_expr(function);
            for argument in &mut arguments {
                *argument = traversal.traverse_expr(*argument);
            }

            traversal.arena().allocate(Expr::Application {
                function,
                arguments,
            })
        }
        Expr::Lambda { arguments, body } => {
            let arguments = arguments.clone();
            let body = *body;

            let body = traversal.traverse_expr(body);

            traversal.arena().allocate(Expr::Lambda { arguments, body })
        }
    }
}
