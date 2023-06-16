use iwc_arena::Idx;
use smol_str::SmolStr;

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
    Variable(SmolStr),
    Unification(usize),
    Function(TyIdx, TyIdx),
    Pair(TyIdx, TyIdx),
}

#[derive(Debug)]
pub enum Constraint {
    Unification(TyIdx, TyIdx),
}
