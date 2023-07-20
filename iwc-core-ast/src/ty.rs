pub mod pretty;
pub mod traversal;

use std::hash::Hash;

use im::Vector;
use iwc_arena::Idx;
use smol_str::SmolStr;

pub type TypeIdx = Idx<Type>;

#[derive(Debug, Clone, Hash)]
pub struct TypeVariableBinder {
    pub name: SmolStr,
}

#[derive(Debug, Clone)]
pub struct Assertion {
    pub name: SmolStr,
    pub arguments: Vector<TypeIdx>,
}

#[derive(Debug, Clone)]
pub enum Type {
    Constructor {
        name: SmolStr,
    },
    Variable {
        name: SmolStr,
        rank: usize,
    },
    Unification {
        name: usize,
    },
    Function {
        arguments: Vector<TypeIdx>,
        result: TypeIdx,
    },
    Application {
        function: TypeIdx,
        arguments: Vector<TypeIdx>,
    },
    Forall {
        variables: Vector<TypeVariableBinder>,
        rank: usize,
        ty: TypeIdx,
    },
    Constrained {
        assertions: Vector<Assertion>,
        ty: TypeIdx,
    },
}

impl Type {
    pub fn is_polymorphic(&self) -> bool {
        matches!(self, Self::Forall { .. })
    }
}

#[derive(Debug, Clone)]
pub struct Instance {
    pub assertion: Assertion,
    pub dependencies: Vector<Assertion>,
}

#[derive(Debug, Clone)]
pub struct FunctionalDependency {
    pub domain: Vector<usize>,
    pub codomain: Vector<usize>,
}

#[derive(Debug, Clone)]
pub struct Class {
    pub functional_dependencies: Vector<FunctionalDependency>,
}
