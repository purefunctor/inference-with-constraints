use std::collections::HashMap;

use iwc_arena::Arena;
use iwc_core_ast::ty::{
    traversal::{default_traverse_ty, Traversal},
    Assertion, Type, TypeIdx, TypeVariableBinder,
};
use iwc_core_constraint::Constraint;
use smol_str::SmolStr;

use crate::context::Context;

pub struct Instantiate<'context> {
    context: &'context mut Context,
}

impl<'context> Instantiate<'context> {
    pub fn new(context: &'context mut Context) -> Self {
        Self { context }
    }

    pub fn instantiate(&mut self, t_idx: TypeIdx) -> TypeIdx {
        if let Type::Forall {
            variables,
            rank,
            ty,
        } = &self.context.volatile.type_arena[t_idx]
        {
            let variables = variables.clone();
            let rank = *rank;
            let ty_idx = *ty;

            if let Type::Constrained { assertions, ty } = &self.context.volatile.type_arena[ty_idx]
            {
                let mut assertions = assertions.clone();
                let ty_idx = *ty;

                let mut substitute =
                    Substitute::from_type_variable_binders(self.context, variables, rank);

                for assertion in assertions.iter_mut() {
                    *assertion = substitute.traverse_assertion(assertion);
                }
                let ty_idx = substitute.traverse_ty(ty_idx);

                for assertion in assertions {
                    self.emit_entail(assertion);
                }

                ty_idx
            } else {
                Substitute::from_type_variable_binders(self.context, variables, rank)
                    .traverse_ty(ty_idx)
            }
        } else {
            t_idx
        }
    }

    fn emit_entail(&mut self, assertion: Assertion) {
        let index = self.context.fresh_index();
        self.context
            .constraints
            .push(Constraint::ClassEntail(index, assertion))
            .unwrap()
    }
}

struct Substitute<'context> {
    context: &'context mut Context,
    substitutions: HashMap<(SmolStr, usize), TypeIdx>,
}

impl<'context> Substitute<'context> {
    fn new(
        context: &'context mut Context,
        substitutions: HashMap<(SmolStr, usize), TypeIdx>,
    ) -> Self {
        Self {
            context,
            substitutions,
        }
    }

    fn from_type_variable_binders<V>(
        context: &'context mut Context,
        variables: V,
        rank: usize,
    ) -> Self
    where
        V: IntoIterator<Item = TypeVariableBinder>,
    {
        let substitutions = variables
            .into_iter()
            .map(|TypeVariableBinder { name }| {
                let index = context.fresh_unification();
                ((name, rank), index)
            })
            .collect();
        Self::new(context, substitutions)
    }
}

impl<'context> Traversal for Substitute<'context> {
    fn arena(&mut self) -> &mut Arena<Type> {
        &mut self.context.volatile.type_arena
    }

    fn traverse_ty(&mut self, ty_idx: TypeIdx) -> TypeIdx {
        match &self.context.volatile.type_arena[ty_idx] {
            Type::Variable { name, rank } => {
                if let Some(&name) = self.substitutions.get(&(name.clone(), *rank)) {
                    name
                } else {
                    ty_idx
                }
            }
            _ => default_traverse_ty(self, ty_idx),
        }
    }
}
