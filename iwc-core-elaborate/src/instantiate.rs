use std::collections::HashMap;

use iwc_arena::Arena;
use iwc_core_ast::ty::{
    traversal::{default_traverse_ty, Traversal},
    Assertion, Type, TypeIdx, TypeVariableBinder,
};
use iwc_core_constraint::Constraint;
use smol_str::SmolStr;

use crate::context::Context;

pub enum InstantiateMode {
    Entail,
    Evidence,
}

pub struct Instantiate<'context> {
    context: &'context mut Context,
}

impl<'context> Instantiate<'context> {
    pub fn new(context: &'context mut Context) -> Self {
        Self { context }
    }

    fn as_substitute(&mut self, variables: Vec<TypeVariableBinder>, rank: usize) -> Substitute {
        Substitute::new(self.context, variables, rank)
    }

    pub fn instantiate(&mut self, t_idx: TypeIdx, mode: InstantiateMode) -> TypeIdx {
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
                let assertions = assertions.clone();
                let ty_idx = *ty;

                let mut substitute = self.as_substitute(variables, rank);

                let assertions = substitute.traverse_assertions(assertions);
                let ty_idx = substitute.traverse_ty(ty_idx);

                for assertion in assertions {
                    match mode {
                        InstantiateMode::Entail => {
                            self.emit_entail(assertion);
                        }
                        InstantiateMode::Evidence => {
                            self.emit_evidence(assertion);
                        }
                    }
                }

                ty_idx
            } else {
                self.as_substitute(variables, rank).traverse_ty(ty_idx)
            }
        } else {
            t_idx
        }
    }

    fn emit_entail(&mut self, assertion: Assertion) {
        self.context
            .constraints
            .push(Constraint::ClassEntail(assertion))
            .unwrap()
    }

    fn emit_evidence(&mut self, assertion: Assertion) {
        self.context
            .constraints
            .push(Constraint::ClassEvidence(assertion))
            .unwrap()
    }
}

struct Substitute<'context> {
    context: &'context mut Context,
    substitutions: HashMap<SmolStr, TypeIdx>,
    rank: usize,
}

impl<'context> Substitute<'context> {
    fn new(
        context: &'context mut Context,
        variables: Vec<TypeVariableBinder>,
        rank: usize,
    ) -> Self {
        let substitutions = variables
            .into_iter()
            .map(|TypeVariableBinder { name }| {
                let index = context.fresh_unification();
                (name, index)
            })
            .collect();
        Self {
            context,
            substitutions,
            rank,
        }
    }
}

impl<'context> Traversal for Substitute<'context> {
    fn arena(&mut self) -> &mut Arena<Type> {
        &mut self.context.volatile.type_arena
    }

    fn traverse_ty(&mut self, ty_idx: TypeIdx) -> TypeIdx {
        match &self.context.volatile.type_arena[ty_idx] {
            Type::Variable { name, rank } if self.rank == *rank => {
                if let Some(&name) = self.substitutions.get(name) {
                    name
                } else {
                    ty_idx
                }
            }
            _ => default_traverse_ty(self, ty_idx),
        }
    }
}
