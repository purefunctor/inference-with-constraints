pub mod common;
pub mod instantiate;

use iwc_arena::Arena;
use iwc_core_ast::ty::Ty;

pub struct Context {
    ty_arena: Arena<Ty>,
    fresh_index: usize,
}
