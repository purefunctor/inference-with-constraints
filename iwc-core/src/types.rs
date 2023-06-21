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
    Variable(SmolStr, usize),
    Unification(usize),
    Function(TyIdx, TyIdx),
    Pair(TyIdx, TyIdx),
    Forall(TypeVariableBindings, usize, TyIdx),
}

impl Ty {
    pub fn is_polymorphic(&self) -> bool {
        matches!(self, Self::Forall(_, _, _))
    }
}

#[derive(Debug)]
pub enum Constraint {
    UnifyDeep(usize, usize),
    UnifySolve(usize, TyIdx),
}
