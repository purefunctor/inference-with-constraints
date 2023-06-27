use iwc_core_ast::ty::{Type, TypeIdx};

impl super::Volatile {
    pub fn fresh_marker(&mut self) -> usize {
        let index = self.fresh_index;
        self.fresh_index += 1;
        index
    }

    pub fn fresh_unification(&mut self) -> TypeIdx {
        let name = self.fresh_marker();
        self.type_arena.allocate(Type::Unification { name })
    }
}
