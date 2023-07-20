pub mod pretty;
pub mod traversal;

use im::Vector;
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
        arguments: Vector<ExprIdx>,
    },
    Lambda {
        arguments: Vector<SmolStr>,
        body: ExprIdx,
    },
}
