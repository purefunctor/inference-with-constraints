use std::iter::zip;

use anyhow::bail;
use iwc_core_ast::{
    expr::{Expr, ExprIdx},
    ty::{pretty::pretty_print_ty, Type, TypeIdx},
};

use crate::instantiate::InstantiateMode;

impl super::Infer {
    pub fn check(&mut self, e_idx: ExprIdx, t_idx: TypeIdx) -> anyhow::Result<()> {
        match (
            &self.volatile.expr_arena[e_idx],
            &self.volatile.type_arena[t_idx],
        ) {
            (Expr::Constructor { name }, _) => {
                let u_idx = self.environment.lookup_constructor(name)?;
                self.unify(t_idx, u_idx);
            }
            (Expr::Variable { name }, _) => {
                let u_idx = self.environment.lookup_value(name)?;
                self.unify(t_idx, u_idx);
            }
            (
                Expr::Application {
                    function,
                    arguments,
                },
                _,
            ) => {
                let function = *function;
                let arguments = arguments.clone();

                let function = self.infer(function)?;
                let function = self.instantiate(function, InstantiateMode::Infer);

                let arguments = arguments
                    .into_iter()
                    .map(|e| self.infer(e))
                    .collect::<anyhow::Result<_>>()?;

                let result = t_idx;

                self.check_function_application(function, arguments, result)?;
            }
            (
                Expr::Lambda {
                    arguments: arguments_expr,
                    body,
                },
                Type::Function {
                    arguments: arguments_ty,
                    result,
                },
            ) => {
                let arguments_expr = arguments_expr.clone();
                let body = *body;

                let arguments_ty = arguments_ty.clone();
                let result = *result;

                for (name, ty) in zip(&arguments_expr, arguments_ty) {
                    self.environment.insert_value(name, ty);
                }

                let body = self.infer(body)?;
                self.unify(body, result);

                for name in &arguments_expr {
                    self.environment.remove_value(name);
                }
            }
            (e, t) => {
                bail!("Invalid check {:?} : {:?}", e, t);
            }
        }

        Ok(())
    }

    fn check_function_application(
        &mut self,
        function: TypeIdx,
        arguments: Vec<TypeIdx>,
        result: TypeIdx,
    ) -> anyhow::Result<()> {
        match &self.volatile.type_arena[function] {
            Type::Function {
                arguments: function_arguments,
                result: function_result,
            } => {
                let function_arguments = function_arguments.clone();
                let function_result = *function_result;

                for (a, b) in zip(arguments, function_arguments) {
                    self.unify(a, b)
                }

                self.unify(result, function_result)
            }
            _ => bail!(
                "{:?}: not a function",
                pretty_print_ty(&self.volatile.type_arena, function)
            ),
        }

        Ok(())
    }
}
