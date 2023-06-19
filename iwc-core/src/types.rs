use iwc_arena::Idx;
use smol_str::SmolStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DeBrujin(pub usize);

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

#[derive(Debug)]
pub enum Ty {
    Unit,
    Variable(DeBrujin),
    Unification(usize),
    Function(TyIdx, TyIdx),
    Pair(TyIdx, TyIdx),
    Forall(DeBrujin, TyIdx),
}

impl Ty {
    pub fn is_polymorphic(&self) -> bool {
        matches!(self, Self::Forall(_, _))
    }
}

#[derive(Debug)]
pub enum Constraint {
    UnifyDeep(usize, usize),
    UnifySolve(usize, TyIdx),
}
