use crate::types::{Expr, ExprIdx};

use super::Context;

/// Helper functions for constructing expressions.
impl Context {
    pub fn expr_unit(&mut self) -> ExprIdx {
        self.ex_arena.allocate(Expr::Unit)
    }

    pub fn expr_variable(&mut self, v: &str) -> ExprIdx {
        self.ex_arena.allocate(Expr::Variable(v.into()))
    }

    pub fn expr_lambda(&mut self, x: &str, v: ExprIdx) -> ExprIdx {
        self.ex_arena.allocate(Expr::Lambda(x.into(), v))
    }

    pub fn expr_application(&mut self, f: ExprIdx, x: ExprIdx) -> ExprIdx {
        self.ex_arena.allocate(Expr::Application(f, x))
    }

    pub fn expr_pair(&mut self, a: ExprIdx, b: ExprIdx) -> ExprIdx {
        self.ex_arena.allocate(Expr::Pair(a, b))
    }
}
