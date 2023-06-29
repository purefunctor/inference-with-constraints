use iwc_core_ast::{
    expr::{Expr, ExprIdx},
    ty::{Type, TypeIdx},
};
use smol_str::SmolStr;

use super::Context;

impl Context {
    pub fn infer(&mut self, e_idx: ExprIdx) -> anyhow::Result<TypeIdx> {
        match &self.volatile.expr_arena[e_idx] {
            Expr::Constructor { name } => self.environment.lookup_constructor_binding(name),
            Expr::Variable { name } => self.environment.lookup_value_binding(name),
            Expr::Application { function, argument } => {
                let function = *function;
                let argument = *argument;

                let function = self.infer(function)?;
                let function = self.instantiate(function);

                let argument = self.infer(argument)?;
                let result = self.volatile.fresh_unification();

                let medium = self
                    .volatile
                    .type_arena
                    .allocate(Type::Function { argument, result });

                self.unify(function, medium)?;

                Ok(result)
            }
            Expr::Lambda { arguments, body } => {
                let arguments = arguments.clone();
                let body = *body;

                let result =
                    self.with_unification_variables(&arguments, |context| context.infer(body))?;

                Ok(arguments.iter().rev().fold(result, |result, _| {
                    let argument = self.volatile.fresh_unification();
                    self.volatile
                        .type_arena
                        .allocate(Type::Function { argument, result })
                }))
            }
        }
    }

    fn with_unification_variables<R>(
        &mut self,
        arguments: &[SmolStr],
        action: impl FnOnce(&mut Self) -> R,
    ) -> R {
        for argument in arguments {
            let unification = self.volatile.fresh_unification();
            self.environment.insert_value_binding(argument, unification)
        }
        let result = action(self);
        for argument in arguments {
            self.environment.remove_value_binding(argument)
        }
        result
    }
}
