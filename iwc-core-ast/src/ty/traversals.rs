use iwc_arena::ArenaLike;

use super::{Assertions, Ty, TyIdx};

pub trait Visitor: Sized {
    type TyArena: ArenaLike<Ty>;

    fn arena(&mut self) -> &mut Self::TyArena;

    fn visit_ty(&mut self, ty: TyIdx) -> TyIdx {
        walk_ty(self, ty)
    }

    fn visit_assertions(&mut self, assertions: Assertions) -> Assertions {
        walk_assertions(self, assertions)
    }
}

pub fn walk_ty<V: Visitor>(visitor: &mut V, ty: TyIdx) -> TyIdx {
    match &visitor.arena()[ty] {
        Ty::Unit => ty,
        Ty::Variable { .. } => ty,
        Ty::Unification { .. } => ty,
        Ty::Function { argument, result } => {
            let argument = *argument;
            let result = *result;

            let argument = visitor.visit_ty(argument);
            let result = visitor.visit_ty(result);

            visitor.arena().allocate(Ty::Function { argument, result })
        }
        Ty::Pair { left, right } => {
            let left = *left;
            let right = *right;

            let left = visitor.visit_ty(left);
            let right = visitor.visit_ty(right);

            visitor.arena().allocate(Ty::Pair { left, right })
        }
        Ty::Forall {
            variables,
            rank,
            ty,
        } => {
            let variables = variables.clone();
            let rank = *rank;
            let ty = *ty;

            let ty = visitor.visit_ty(ty);

            visitor.arena().allocate(Ty::Forall {
                variables,
                rank,
                ty,
            })
        }
        Ty::Constrained { assertions, ty } => {
            let assertions = assertions.clone();
            let ty = *ty;

            let assertions = visitor.visit_assertions(assertions);
            let ty = visitor.visit_ty(ty);

            visitor.arena().allocate(Ty::Constrained { assertions, ty })
        }
    }
}

pub fn walk_assertions<V: Visitor>(visitor: &mut V, mut assertions: Assertions) -> Assertions {
    for assertion in &mut assertions {
        for argument in &mut assertion.arguments {
            *argument = visitor.visit_ty(*argument)
        }
    }
    assertions
}
