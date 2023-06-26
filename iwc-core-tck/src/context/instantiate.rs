//! Implements the instantiation algorithm.
//!
//! Instantiation replaces type variables introduced by a `forall` with unification variables.
//! Class constraints are passed to the inference constraint solver for the entailment algorithm.
//!
//! # Examples
//!
//! ```haskell
//! -- id
//! instantiate(forall a. a -> a) = [?0 -> ?0]
//!
//! -- const
//! instantiate(forall a b. a -> b -> a) = [?0 -> ?1 -> ?0]
//!
//! -- eq
//! instantiate(forall a. Eq a => a -> a -> Boolean) = [Eq ?0 => ?0 -> ?0] = [?0 -> ?0]
//! ```
//!
//! # Higher-Rank Types
//!
//! The instantiation algorithm also takes into account higher-rank polymorphism:
//!
//! ```haskell
//! instantiate((forall a. a -> a) -> ()) = [(forall a. a -> a) -> ()]
//! ```

use iwc_arena::Arena;
use iwc_core_ast::ty::{self, Assertions, Ty, TyIdx, TypeVariableBindings, Visitor};

use super::Context;

impl Context {
    pub fn instantiate_type(&mut self, ty: TyIdx) -> (Assertions, TyIdx) {
        if let Ty::Forall {
            variables,
            rank,
            ty,
        } = &self.volatile.ty_arena[ty]
        {
            let variables = variables.clone();
            let rank = *rank;
            let ty = *ty;

            if let Ty::Constrained { assertions, ty } = &self.volatile.ty_arena[ty] {
                let assertions = assertions.clone();
                let ty = *ty;

                let mut visitor = Instantiate::new(self, variables, rank);

                let assertions = visitor.visit_assertions(assertions);
                let ty = visitor.visit_ty(ty);

                return (assertions, ty);
            }

            let mut visitor = Instantiate::new(self, variables, rank);

            return (Assertions::new(), visitor.visit_ty(ty));
        }

        return (Assertions::new(), ty);
    }
}

struct Instantiate<'context> {
    context: &'context mut Context,
    variables: TypeVariableBindings,
    rank: usize,
}

impl<'context> Instantiate<'context> {
    pub fn new(
        context: &'context mut Context,
        variables: TypeVariableBindings,
        rank: usize,
    ) -> Self {
        Self {
            context,
            variables,
            rank,
        }
    }
}

impl<'context> ty::Visitor for Instantiate<'context> {
    fn arena(&mut self) -> &mut Arena<Ty> {
        &mut self.context.volatile.ty_arena
    }

    fn visit_ty(&mut self, ty: TyIdx) -> TyIdx {
        match &self.context.volatile.ty_arena[ty] {
            Ty::Variable { name, rank } if self.rank == *rank && self.variables.contains(name) => {
                self.context.fresh_unification_variable()
            }
            _ => ty::walk_ty(self, ty),
        }
    }
}
