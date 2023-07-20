use iwc_arena::Arena;

use crate::ty::{Assertion, Type, TypeIdx};

pub trait Traversal: Sized {
    fn arena(&mut self) -> &mut Arena<Type>;

    fn traverse_ty(&mut self, ty_idx: TypeIdx) -> TypeIdx {
        default_traverse_ty(self, ty_idx)
    }

    fn traverse_assertion(&mut self, assertion: &Assertion) -> Assertion {
        default_traverse_assertion(self, assertion)
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

            for argument in arguments.iter_mut() {
                *argument = traversal.traverse_ty(*argument);
            }
            let result = traversal.traverse_ty(result);

            traversal
                .arena()
                .allocate(Type::Function { arguments, result })
        }
        Type::Application {
            function,
            arguments,
        } => {
            let function = *function;
            let mut arguments = arguments.clone();

            let function = traversal.traverse_ty(function);
            for argument in arguments.iter_mut() {
                *argument = traversal.traverse_ty(*argument);
            }

            traversal.arena().allocate(Type::Application {
                function,
                arguments,
            })
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
            let mut assertions = assertions.clone();
            let ty = *ty;

            for assertion in assertions.iter_mut() {
                *assertion = traversal.traverse_assertion(&assertion)
            }
            let ty = traversal.traverse_ty(ty);

            traversal
                .arena()
                .allocate(Type::Constrained { assertions, ty })
        }
    }
}

pub fn default_traverse_assertion<T: Traversal>(
    traversal: &mut T,
    assertion: &Assertion,
) -> Assertion {
    let mut assertion = assertion.clone();
    for argument in assertion.arguments.iter_mut() {
        *argument = traversal.traverse_ty(*argument);
    }
    assertion
}
