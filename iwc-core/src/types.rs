pub mod traversals;

use iwc_arena::Idx;
use smol_str::SmolStr;
use tinyvec::TinyVec;

const INLINE_LIMIT: usize = 4;

pub type ExprIdx = Idx<Expr>;

#[derive(Debug)]
pub enum Expr {
    Unit,
    Variable(SmolStr),
    Lambda(SmolStr, ExprIdx),
    Application(ExprIdx, ExprIdx),
    Pair(ExprIdx, ExprIdx),
}

pub type SuperClassHead = TinyVec<[usize; INLINE_LIMIT]>;

#[derive(Default)]
pub struct SuperClass {
    pub name: SmolStr,
    pub arguments: SuperClassHead,
}

pub type SuperClasses = TinyVec<[SuperClass; INLINE_LIMIT]>;

pub type ClassHead = TinyVec<[SmolStr; INLINE_LIMIT]>;

pub struct Class {
    pub name: SmolStr,
    pub arguments: ClassHead,
    pub superclasses: SuperClasses,
}

pub type AssertionHead = TinyVec<[TyIdx; INLINE_LIMIT]>;

#[derive(Clone, Debug, Default)]
pub struct Assertion {
    pub name: SmolStr,
    pub arguments: AssertionHead,
}

pub type Assertions = TinyVec<[Assertion; INLINE_LIMIT]>;

pub struct Instance {
    pub assertion: Assertion,
    pub dependencies: Assertions,
}

pub type TyIdx = Idx<Ty>;

pub type TypeVariableBindings = TinyVec<[SmolStr; INLINE_LIMIT]>;

#[derive(Debug)]
pub enum Ty {
    Unit,
    Variable {
        name: SmolStr,
        rank: usize,
    },
    Unification {
        value: usize,
    },
    Function {
        argument: TyIdx,
        result: TyIdx,
    },
    Pair {
        left: TyIdx,
        right: TyIdx,
    },
    Forall {
        variables: TypeVariableBindings,
        rank: usize,
        ty: TyIdx,
    },
    Constrained {
        assertions: Assertions,
        ty: TyIdx,
    },
}

impl Ty {
    pub fn is_polymorphic(&self) -> bool {
        matches!(self, Self::Forall { .. })
    }
}

#[derive(Debug)]
pub enum Constraint {
    Assertion(SmolStr, AssertionHead),
    UnifyDeep(usize, usize),
    UnifySolve(usize, TyIdx),
}
