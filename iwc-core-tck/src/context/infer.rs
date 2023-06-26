//! Implements the inference algorithm.

use anyhow::Ok;
use iwc_core_ast::{
    expr::{Expr, ExprIdx},
    ty::{Assertions, Ty, TyIdx},
};

use super::Constraint;

impl super::Context {
    pub fn infer(&mut self, e_idx: ExprIdx) -> anyhow::Result<TyIdx> {
        match &self.volatile.expr_arena[e_idx] {
            Expr::Unit => Ok(self.volatile.ty_arena.allocate(Ty::Unit)),
            Expr::Variable { name } => self.lookup_variable(name),
            Expr::Lambda { name, body } => {
                let name = name.clone();
                let body = *body;

                let argument = self.fresh_unification_variable();
                let result =
                    self.with_bound_variable(&name, argument, |context| context.infer(body))?;

                Ok(self
                    .volatile
                    .ty_arena
                    .allocate(Ty::Function { argument, result }))
            }
            Expr::Application { function, argument } => {
                let function = *function;
                let argument = *argument;

                let function = self.infer(function)?;
                let (assertions, function) = self.instantiate_type(function);

                let argument = self.infer(argument)?;
                let result = self.fresh_unification_variable();
                let medium = self
                    .volatile
                    .ty_arena
                    .allocate(Ty::Function { argument, result });

                self.unify(function, medium)?;
                self.emit_assertions(assertions);

                Ok(result)
            }
            Expr::Pair { left, right } => {
                let left = *left;
                let right = *right;

                let left = self.infer(left)?;
                let right = self.infer(right)?;

                Ok(self.volatile.ty_arena.allocate(Ty::Pair { left, right }))
            }
        }
    }

    fn emit_assertions(&mut self, assertions: Assertions) {
        for assertion in assertions {
            let marker = self.fresh_marker();
            self.volatile
                .constraints
                .push(Constraint::ClassAssertion(marker, assertion));
        }
    }
}
