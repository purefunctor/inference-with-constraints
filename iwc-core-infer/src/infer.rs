use iwc_core_ast::{
    expr::{Expr, ExprIdx},
    ty::{Type, TypeIdx},
};
use smol_str::SmolStr;

use crate::instantiate::InstantiateMode;

impl super::Infer {
    pub fn infer(&mut self, e_idx: ExprIdx) -> anyhow::Result<TypeIdx> {
        match &self.volatile.expr_arena[e_idx] {
            Expr::Constructor { name } => self.environment.lookup_constructor(name),
            Expr::Variable { name } => self.environment.lookup_value(name),
            Expr::Application {
                function,
                arguments,
            } => {
                let function = *function;
                let arguments = arguments.clone();

                let function = self.infer(function)?;
                let function = self.instantiate(function, InstantiateMode::Infer);

                let arguments = arguments
                    .into_iter()
                    .map(|argument| self.infer(argument))
                    .collect::<anyhow::Result<_>>()?;
                let result = self.volatile.fresh_unification();

                let medium = self
                    .volatile
                    .type_arena
                    .allocate(Type::Function { arguments, result });

                self.unify(function, medium);

                Ok(result)
            }
            Expr::Lambda { arguments, body } => {
                let arguments = arguments.clone();
                let body = *body;

                let variables: Vec<_> = arguments
                    .into_iter()
                    .map(|name| (name, self.volatile.fresh_unification()))
                    .collect();

                let result =
                    self.with_unification_variables(&variables, |context| context.infer(body))?;

                let arguments = variables
                    .into_iter()
                    .map(|(_, argument)| argument)
                    .collect();

                Ok(self
                    .volatile
                    .type_arena
                    .allocate(Type::Function { arguments, result }))
            }
        }
    }

    fn with_unification_variables<R>(
        &mut self,
        variables: &[(SmolStr, TypeIdx)],
        action: impl FnOnce(&mut Self) -> R,
    ) -> R {
        for (variable, unification) in variables {
            self.environment
                .insert_value(variable, *unification)
        }
        let result = action(self);
        for (variable, _) in variables {
            self.environment.remove_value(variable)
        }
        result
    }
}
