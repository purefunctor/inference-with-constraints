use crate::types::{
    traversals::{walk_ty, Visitor},
    Assertions, Ty, TyIdx, TypeVariableBindings,
};

use super::Context;

struct Instantiation<'context> {
    context: &'context mut Context,
    variables: TypeVariableBindings,
    rank: usize,
}

impl<'context> Instantiation<'context> {
    fn new(context: &'context mut Context, variables: TypeVariableBindings, rank: usize) -> Self {
        Self {
            context,
            variables,
            rank,
        }
    }
}

impl<'context> Visitor for Instantiation<'context> {
    fn context(&mut self) -> &mut Context {
        self.context
    }

    fn visit_ty(&mut self, ty: TyIdx) -> TyIdx {
        match &self.context.ty_arena[ty] {
            Ty::Variable { name, rank } => {
                if self.rank == *rank && self.variables.contains(name) {
                    self.context.ty_unification_fresh()
                } else {
                    ty
                }
            }
            _ => walk_ty(self, ty),
        }
    }
}

impl Context {
    pub fn instantiate_type(&mut self, ty: TyIdx) -> (Assertions, TyIdx) {
        if let Ty::Forall {
            variables,
            rank,
            ty,
        } = &self.ty_arena[ty]
        {
            let variables = variables.clone();
            let rank = *rank;
            let ty = *ty;

            if let Ty::Constrained { assertions, ty } = &self.ty_arena[ty] {
                let assertions = assertions.clone();
                let ty = *ty;

                let mut visitor = Instantiation::new(self, variables, rank);

                let assertions = visitor.visit_assertions(assertions);
                let ty = visitor.visit_ty(ty);

                return (assertions, ty);
            }

            let visitor = &mut Instantiation::new(self, variables, rank);

            return (Assertions::new(), visitor.visit_ty(ty));
        }

        return (Assertions::new(), ty);
    }
}
