use anyhow::Context;
use im::Vector;
use iwc_core_ast::{
    expr::{Expr, ExprIdx},
    ty::{Type, TypeIdx},
};
use smol_str::SmolStr;

use crate::{instantiate::Instantiate, solve::Solve, unify::Unify};

pub struct Infer<'context> {
    context: &'context mut crate::context::Context,
}

impl<'context> Infer<'context> {
    pub fn new(context: &'context mut crate::context::Context) -> Self {
        Self { context }
    }

    pub fn to_solve(self) -> Solve<'context> {
        Solve::new(self.context)
    }

    pub fn as_unify<'infer>(&'infer mut self) -> Unify<'infer> {
        Unify::new(self.context)
    }

    pub fn as_instantiate<'infer>(&'infer mut self) -> Instantiate<'infer> {
        Instantiate::new(self.context)
    }

    pub fn infer(&mut self, e_idx: ExprIdx) -> anyhow::Result<TypeIdx> {
        match &self.context.volatile.expr_arena[e_idx] {
            Expr::Constructor { name } => self
                .context
                .environment
                .constructors
                .get(name)
                .copied()
                .context(format!("Could not find constructor: {:?}", name)),
            Expr::Variable { name } => self
                .context
                .environment
                .values
                .get(name)
                .copied()
                .context(format!("Could not find value: {:?}", name)),
            Expr::Application {
                function,
                arguments,
            } => {
                let function = *function;
                let arguments = arguments.clone();

                let function = self.infer(function)?;
                let function = self.as_instantiate().instantiate(function);

                let arguments: Vector<TypeIdx> = arguments
                    .into_iter()
                    .map(|argument| self.infer(argument))
                    .collect::<anyhow::Result<_>>()?;
                let result = self.context.fresh_unification();

                let medium = self
                    .context
                    .volatile
                    .type_arena
                    .allocate(Type::Function { arguments, result });

                self.as_unify().unify(function, medium);

                Ok(result)
            }
            Expr::Lambda { arguments, body } => {
                let arguments = arguments.clone();
                let body = *body;

                let variables: Vec<_> = arguments
                    .into_iter()
                    .map(|name| (name, self.context.fresh_unification()))
                    .collect();

                let result =
                    self.with_unification_variables(&variables, |infer| infer.infer(body))?;

                let arguments = variables
                    .into_iter()
                    .map(|(_, argument)| argument)
                    .collect();

                Ok(self
                    .context
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
            self.context
                .environment
                .values
                .insert(variable.clone(), *unification);
        }
        let result = action(self);
        for (variable, _) in variables {
            self.context.environment.values.remove(variable);
        }
        result
    }
}
