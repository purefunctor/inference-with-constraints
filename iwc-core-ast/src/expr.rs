mod traversals;

pub use traversals::*;

use iwc_arena::Idx;
use smol_str::SmolStr;

pub type ExprIdx = Idx<Expr>;

#[derive(Debug)]
pub enum Expr {
    Unit,
    Variable {
        name: SmolStr,
    },
    Lambda {
        name: SmolStr,
        body: ExprIdx,
    },
    Application {
        function: ExprIdx,
        argument: ExprIdx,
    },
    Pair {
        left: ExprIdx,
        right: ExprIdx,
    },
}
