use std::fmt::Write;

use iwc_arena::Arena;

use crate::ty::{Type, TypeIdx, TypeVariableBinder};

pub fn pretty_print(type_arena: &Arena<Type>, ty_idx: TypeIdx) -> String {
    match &type_arena[ty_idx] {
        Type::Constructor { name } => format!("{}", name),
        Type::Variable { name, rank } => format!("{}@{}", name, rank),
        Type::Unification { name } => format!("?{}", name),
        Type::Function { argument, result } => format!(
            "({} -> {})",
            pretty_print(type_arena, *argument),
            pretty_print(type_arena, *result)
        ),
        Type::Application { function, argument } => format!(
            "({} {})",
            pretty_print(type_arena, *function),
            pretty_print(type_arena, *argument),
        ),
        Type::Forall {
            variables,
            rank,
            ty,
        } => {
            let mut result = String::new();
            write!(result, "forall ").unwrap();
            for TypeVariableBinder { name } in variables {
                write!(result, "{}", name).unwrap();
            }
            write!(result, "@ {}. ", rank).unwrap();
            write!(result, "{}", pretty_print(type_arena, *ty)).unwrap();
            result
        }
        Type::Constrained { assertions, ty } => pretty_print(type_arena, *ty),
    }
}
