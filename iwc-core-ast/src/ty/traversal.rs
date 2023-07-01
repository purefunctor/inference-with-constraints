use iwc_arena::Arena;

use crate::ty::{Assertion, Type, TypeIdx};

pub trait Traversal: Sized {
    fn arena(&mut self) -> &mut Arena<Type>;

    fn traverse_ty(&mut self, ty_idx: TypeIdx) -> TypeIdx {
        default_traverse_ty(self, ty_idx)
    }

    fn traverse_assertions(&mut self, assertions: Vec<Assertion>) -> Vec<Assertion> {
        default_traverse_assertions(self, assertions)
    }
}

pub fn default_traverse_ty<T: Traversal>(traversal: &mut T, ty_idx: TypeIdx) -> TypeIdx {
    match &traversal.arena()[ty_idx] {
        Type::Constructor { .. } => ty_idx,
        Type::Variable { .. } => ty_idx,
        Type::Unification { .. } => ty_idx,
        Type::Function { arguments, result } => {
            let mut arguments = arguments.clone();
            let result = *result;

            for argument in &mut arguments {
                *argument = traversal.traverse_ty(*argument);
            }
            let result = traversal.traverse_ty(result);

            traversal
                .arena()
                .allocate(Type::Function { arguments, result })
        }
        Type::Application { function, argument } => {
            let function = *function;
            let argument = *argument;

            let function = traversal.traverse_ty(function);
            let argument = traversal.traverse_ty(argument);

            traversal
                .arena()
                .allocate(Type::Application { function, argument })
        }
        Type::Forall {
            variables,
            rank,
            ty,
        } => {
            let variables = variables.clone();
            let rank = *rank;
            let ty = *ty;

            // TODO: kinded variable binders need to be traversed.
            let ty = traversal.traverse_ty(ty);

            traversal.arena().allocate(Type::Forall {
                variables,
                rank,
                ty,
            })
        }
        Type::Constrained { assertions, ty } => {
            let assertions = assertions.clone();
            let ty = *ty;

            let assertions = traversal.traverse_assertions(assertions);
            let ty = traversal.traverse_ty(ty);

            traversal
                .arena()
                .allocate(Type::Constrained { assertions, ty })
        }
    }
}

pub fn default_traverse_assertions<T: Traversal>(
    traversal: &mut T,
    mut assertions: Vec<Assertion>,
) -> Vec<Assertion> {
    for assertion in &mut assertions {
        for argument in &mut assertion.arguments {
            *argument = traversal.traverse_ty(*argument);
        }
    }
    assertions
}
