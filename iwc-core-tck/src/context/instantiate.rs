use std::collections::HashMap;

use iwc_core_ast::{
    traversal::ty::{default_traverse_ty, Traversal},
    ty::{Type, TypeIdx, TypeVariableBinder},
};
use smol_str::SmolStr;

use super::{Constraint, Context, Volatile};

type Instantiations = HashMap<SmolStr, TypeIdx>;

impl Context {
    pub fn instantiate(&mut self, t_idx: TypeIdx) -> TypeIdx {
        if let Type::Forall {
            variables,
            rank,
            ty,
        } = &self.volatile.type_arena[t_idx]
        {
            let variables = variables.clone();
            let rank = *rank;
            let ty = *ty;

            if let Type::Constrained { assertions, ty } = &self.volatile.type_arena[ty] {
                let assertions = assertions.clone();
                let ty = *ty;

                let mut instantiate = Instantiate::new(&mut self.volatile, variables, rank);

                let assertions = instantiate.traverse_assertions(assertions);
                let ty = instantiate.traverse_ty(ty);

                for assertion in assertions {
                    self.volatile
                        .constraints
                        .push(Constraint::ClassAssertion(assertion))
                }

                ty
            } else {
                Instantiate::new(&mut self.volatile, variables, rank).traverse_ty(ty)
            }
        } else {
            t_idx
        }
    }
}

struct Instantiate<'a> {
    volatile: &'a mut Volatile,
    instantiations: Instantiations,
    rank: usize,
}

impl<'a> Instantiate<'a> {
    fn new(volatile: &'a mut Volatile, variables: Vec<TypeVariableBinder>, rank: usize) -> Self {
        let instantiations = variables
            .into_iter()
            .map(|TypeVariableBinder { name }| {
                let index = volatile.fresh_unification();
                (name, index)
            })
            .collect();
        Self {
            volatile,
            instantiations,
            rank,
        }
    }
}

impl<'a> Traversal for Instantiate<'a> {
    fn arena(&mut self) -> &mut iwc_arena::Arena<Type> {
        &mut self.volatile.type_arena
    }

    fn traverse_ty(&mut self, ty_idx: TypeIdx) -> TypeIdx {
        match &self.volatile.type_arena[ty_idx] {
            Type::Variable { name, rank } if self.rank == *rank => {
                if let Some(&name) = self.instantiations.get(name) {
                    name
                } else {
                    ty_idx
                }
            }
            _ => default_traverse_ty(self, ty_idx),
        }
    }
}
