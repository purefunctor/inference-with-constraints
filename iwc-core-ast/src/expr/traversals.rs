use iwc_arena::Arena;

use super::{Expr, ExprIdx};

pub trait Visitor: Sized {
    fn arena(&mut self) -> &mut Arena<Expr>;

    fn visit_expr(&mut self, expr: ExprIdx) -> ExprIdx {
        walk_expr(self, expr)
    }
}

pub fn walk_expr<V: Visitor>(visitor: &mut V, expr: ExprIdx) -> ExprIdx {
    match &visitor.arena()[expr] {
        Expr::Unit => expr,
        Expr::Variable { .. } => expr,
        Expr::Lambda { argument, body } => {
            let argument = argument.clone();
            let body = *body;

            let body = visitor.visit_expr(body);

            visitor.arena().allocate(Expr::Lambda { argument, body })
        }
        Expr::Application { function, argument } => {
            let function = *function;
            let argument = *argument;

            let function = visitor.visit_expr(function);
            let argument = visitor.visit_expr(argument);

            visitor
                .arena()
                .allocate(Expr::Application { function, argument })
        }
        Expr::Pair { left, right } => {
            let left = *left;
            let right = *right;

            let left = visitor.visit_expr(left);
            let right = visitor.visit_expr(right);

            visitor.arena().allocate(Expr::Pair { left, right })
        }
    }
}
