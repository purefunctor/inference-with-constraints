pub mod pretty;
pub mod traversal;

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
        arguments: Vec<ExprIdx>,
    },
    Lambda {
        arguments: Vec<SmolStr>,
        body: ExprIdx,
    },
}
