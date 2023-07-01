use std::iter::zip;

use anyhow::bail;
use iwc_core_ast::ty::{Assertion, Type, TypeIdx};

use super::{Constraint, Context};

impl Context {
    pub fn unify(&mut self, t_idx: TypeIdx, u_idx: TypeIdx) -> anyhow::Result<()> {
        match (
            &self.volatile.type_arena[t_idx],
            &self.volatile.type_arena[u_idx],
        ) {
            // Constructor
            (Type::Constructor { name: t_name }, Type::Constructor { name: u_name })
                if t_name == u_name => {}
            // Variable
            (
                Type::Variable {
                    name: t_name,
                    rank: t_rank,
                },
                Type::Variable {
                    name: u_name,
                    rank: u_rank,
                },
            ) if t_name == u_name && t_rank == u_rank => (),
            // Unification
            (Type::Unification { name: t_name }, Type::Unification { name: u_name }) => {
                if t_name != u_name {
                    self.emit_deep(*t_name, *u_name);
                }
            }
            // Left-Solve
            (t_ty, Type::Unification { name: u_name }) => {
                if t_ty.is_polymorphic() {
                    bail!(
                        "Impredicative type while unifying {:?} ~ {:?}",
                        t_ty,
                        u_name
                    );
                }
                if self.occurs_check(t_idx, *u_name) {
                    bail!("Infinite type while unifying {:?} ~ {:?}", t_ty, u_name);
                }
                self.emit_solve(*u_name, t_idx);
            }
            // Right-Solve
            (Type::Unification { name: t_name }, u_ty) => {
                if u_ty.is_polymorphic() {
                    bail!(
                        "Impredicative type while unifying {:?} ~ {:?}",
                        t_name,
                        u_ty
                    );
                }
                if self.occurs_check(u_idx, *t_name) {
                    bail!("Infinite type while unifying {:?} ~ {:?}", t_name, u_ty);
                }
                self.emit_solve(*t_name, u_idx)
            }
            // Function
            (
                Type::Function {
                    arguments: t_arguments,
                    result: t_result,
                },
                Type::Function {
                    arguments: u_arguments,
                    result: u_result,
                },
            ) => {
                let t_arguments = t_arguments.clone();
                let u_arguments = u_arguments.clone();

                let t_result = *t_result;
                let u_result = *u_result;

                for (t_argument, u_argument) in zip(t_arguments, u_arguments) {
                    self.unify(t_argument, u_argument)?;
                }

                self.unify(t_result, u_result)?;
            }
            // Application
            (
                Type::Application {
                    function: t_function,
                    argument: t_argument,
                },
                Type::Application {
                    function: u_function,
                    argument: u_argument,
                },
            ) => {
                let t_function = *t_function;
                let u_function = *u_function;

                let t_argument = *t_argument;
                let u_argument = *u_argument;

                self.unify(t_function, u_function)?;
                self.unify(t_argument, u_argument)?;
            }
            (t_ty, u_ty) => {
                bail!("Cannot unify {:?} ~ {:?}", t_ty, u_ty);
            }
        }

        Ok(())
    }

    fn occurs_check(&self, t_idx: TypeIdx, u_name: usize) -> bool {
        match &self.volatile.type_arena[t_idx] {
            Type::Constructor { .. } => false,
            Type::Variable { .. } => false,
            Type::Unification { name: t_name } => *t_name == u_name,
            Type::Function { arguments, result } => {
                arguments
                    .iter()
                    .any(|argument| self.occurs_check(*argument, u_name))
                    || self.occurs_check(*result, u_name)
            }
            Type::Application { function, argument } => {
                self.occurs_check(*function, u_name) || self.occurs_check(*argument, u_name)
            }
            Type::Forall { ty, .. } => self.occurs_check(*ty, u_name),
            Type::Constrained { assertions, ty } => {
                assertions.iter().any(|Assertion { arguments, .. }| {
                    arguments
                        .iter()
                        .any(|argument| self.occurs_check(*argument, u_name))
                }) || self.occurs_check(*ty, u_name)
            }
        }
    }

    fn emit_deep(&mut self, t_name: usize, u_name: usize) {
        self.volatile
            .constraints
            .push(Constraint::UnifyDeep(t_name, u_name))
    }

    fn emit_solve(&mut self, t_name: usize, u_idx: TypeIdx) {
        self.volatile
            .constraints
            .push(Constraint::UnifySolve(t_name, u_idx))
    }
}
