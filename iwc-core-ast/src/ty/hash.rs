use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use iwc_arena::Arena;

use super::{Assertion, Type, TypeIdx};

pub fn hash_ty(arena: &Arena<Type>, t_idx: TypeIdx) -> u64 {
    let ref mut state = DefaultHasher::default();
    hash_ty_core(state, arena, t_idx);
    state.finish()
}

pub fn hash_assertion(arena: &Arena<Type>, assertion: &Assertion) -> u64 {
    let ref mut state = DefaultHasher::default();
    hash_assertion_core(state, arena, assertion);
    state.finish()
}

fn hash_ty_core<H: Hasher>(state: &mut H, arena: &Arena<Type>, t_idx: TypeIdx) {
    match &arena[t_idx] {
        Type::Constructor { name } => name.hash(state),
        Type::Variable { name, rank } => {
            name.hash(state);
            rank.hash(state);
        }
        Type::Unification { name } => name.hash(state),
        Type::Function { arguments, result } => {
            for argument in arguments {
                hash_ty_core(state, arena, *argument);
            }
            hash_ty_core(state, arena, *result);
        }
        Type::Application { function, argument } => {
            hash_ty_core(state, arena, *function);
            hash_ty_core(state, arena, *argument);
        }
        Type::Forall {
            variables,
            rank,
            ty,
        } => {
            for variable in variables {
                variable.hash(state);
            }
            rank.hash(state);
            hash_ty_core(state, arena, *ty);
        }
        Type::Constrained { assertions, ty } => {
            for assertion in assertions {
                hash_assertion_core(state, arena, assertion);
            }
            hash_ty_core(state, arena, *ty);
        }
    }
}

fn hash_assertion_core<H: Hasher>(state: &mut H, arena: &Arena<Type>, assertion: &Assertion) {
    assertion.name.hash(state);
    for argument in &assertion.arguments {
        hash_ty_core(state, arena, *argument);
    }
}
