pub mod pretty;
pub mod traversal;

use iwc_arena::Idx;
use smol_str::SmolStr;

pub type TypeIdx = Idx<Type>;

#[derive(Debug, Clone)]
pub struct TypeVariableBinder {
    pub name: SmolStr,
}

#[derive(Debug, Clone)]
pub struct Assertion {
    pub name: SmolStr,
    pub arguments: Vec<TypeIdx>,
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
        arguments: Vec<TypeIdx>,
        result: TypeIdx,
    },
    Application {
        function: TypeIdx,
        argument: TypeIdx,
    },
    Forall {
        variables: Vec<TypeVariableBinder>,
        rank: usize,
        ty: TypeIdx,
    },
    Constrained {
        assertions: Vec<Assertion>,
        ty: TypeIdx,
    },
}

impl Type {
    pub fn is_polymorphic(&self) -> bool {
        matches!(self, Self::Forall { .. })
    }
}
