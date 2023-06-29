use iwc_arena::Idx;
use smol_str::SmolStr;

pub type ExprIdx = Idx<Expr>;

#[derive(Debug, Clone)]
pub enum Expr {
    Constructor {
        name: SmolStr,
    },
    Variable {
        name: SmolStr,
    },
    Application {
        function: ExprIdx,
        argument: ExprIdx,
    },
    Lambda {
        arguments: Vec<SmolStr>,
        body: ExprIdx,
    },
}
