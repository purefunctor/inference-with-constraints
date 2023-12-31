use std::fmt::Write;

use iwc_arena::Arena;

use crate::ty::{Type, TypeIdx, TypeVariableBinder};

use super::Assertion;

pub fn pretty_print_ty(type_arena: &Arena<Type>, ty_idx: TypeIdx) -> String {
    match &type_arena[ty_idx] {
        Type::Constructor { name } => format!("{}", name),
        Type::Variable { name, rank } => format!("{}_{}", name, rank),
        Type::Unification { name } => format!("?{}", name),
        Type::Function { arguments, result } => {
            let mut accumulator = String::new();
            for argument in arguments {
                write!(
                    accumulator,
                    "{} -> ",
                    pretty_print_ty(type_arena, *argument)
                )
                .unwrap();
            }
            write!(accumulator, "{}", pretty_print_ty(type_arena, *result)).unwrap();
            accumulator
        }
        Type::Application {
            function,
            arguments,
        } => {
            let mut result = String::new();
            write!(result, "({}", pretty_print_ty(type_arena, *function)).unwrap();
            for argument in arguments {
                write!(result, " {}", pretty_print_ty(type_arena, *argument)).unwrap();
            }
            write!(result, ")").unwrap();
            result
        }
        Type::Forall {
            variables,
            rank,
            ty,
        } => {
            let mut result = String::new();
            write!(result, "(forall_{} ", rank).unwrap();
            for TypeVariableBinder { name } in variables {
                write!(result, "{}", name).unwrap();
            }
            write!(result, ". {})", pretty_print_ty(type_arena, *ty)).unwrap();
            result
        }
        Type::Constrained { assertions, ty } => {
            let mut result = String::new();

            let mut assertions = assertions.iter().peekable();

            write!(result, "(").unwrap();
            while let Some(assertion) = assertions.next() {
                write!(result, "{}", pretty_print_assertion(type_arena, assertion)).unwrap();
                if assertions.peek().is_some() {
                    write!(result, ", ").unwrap();
                }
            }
            write!(result, ") => ").unwrap();

            write!(result, "{}", pretty_print_ty(type_arena, *ty)).unwrap();

            result
        }
    }
}

pub fn pretty_print_assertion(type_arena: &Arena<Type>, assertion: &Assertion) -> String {
    let mut result = String::new();

    write!(result, "{}", assertion.name).unwrap();
    for argument in assertion.arguments.iter() {
        write!(result, " {}", pretty_print_ty(type_arena, *argument)).unwrap();
    }

    result
}
