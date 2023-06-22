use iwc_arena::Idx;
use smol_str::SmolStr;
use tinyvec::TinyVec;

pub type ExprIdx = Idx<Expr>;

#[derive(Debug)]
pub enum Expr {
    Unit,
    Variable(SmolStr),
    Lambda(SmolStr, ExprIdx),
    Application(ExprIdx, ExprIdx),
    Pair(ExprIdx, ExprIdx),
}

pub type TyIdx = Idx<Ty>;

pub type TypeVariableBindings = TinyVec<[SmolStr; 4]>;

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
}

impl Ty {
    pub fn is_polymorphic(&self) -> bool {
        matches!(self, Self::Forall { .. })
    }
}

#[derive(Debug)]
pub enum Constraint {
    UnifyDeep(usize, usize),
    UnifySolve(usize, TyIdx),
}
