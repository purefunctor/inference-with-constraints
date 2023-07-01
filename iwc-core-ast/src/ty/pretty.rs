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
        Type::Application { function, argument } => format!(
            "({} {})",
            pretty_print_ty(type_arena, *function),
            pretty_print_ty(type_arena, *argument),
        ),
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
            format!(
                "{} => {}",
                pretty_print_assertions(type_arena, assertions),
                pretty_print_ty(type_arena, *ty)
            )
        }
    }
}

pub fn pretty_print_assertions(type_arena: &Arena<Type>, assertions: &[Assertion]) -> String {
    let mut result = String::new();

    let mut assertions = assertions.iter().peekable();

    write!(result, "(").unwrap();
    while let Some(assertion) = assertions.next() {
        write!(result, "{}", assertion.name).unwrap();
        let mut arguments = assertion.arguments.iter().peekable();
        while let Some(argument) = arguments.next() {
            write!(result, " {}", pretty_print_ty(type_arena, *argument)).unwrap();
            if arguments.peek().is_some() {
                write!(result, ", ").unwrap();
            }
        }
        if assertions.peek().is_some() {
            write!(result, ", ").unwrap();
        }
    }
    write!(result, ")").unwrap();

    result
}
