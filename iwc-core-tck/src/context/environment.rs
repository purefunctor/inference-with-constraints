use anyhow::Context;
use iwc_core_ast::ty::TypeIdx;

impl super::Environment {
    pub fn lookup_value_binding(&mut self, key: &str) -> anyhow::Result<TypeIdx> {
        self.value_bindings
            .get(key)
            .context(format!("No binding found {:?}", key))
            .copied()
    }

    pub fn insert_value_binding(&mut self, key: &str, value: TypeIdx) {
        self.value_bindings.insert(key.into(), value);
    }

    pub fn remove_value_binding(&mut self, key: &str) {
        self.value_bindings.remove(key);
    }

    pub fn lookup_constructor_binding(&mut self, key: &str) -> anyhow::Result<TypeIdx> {
        self.constructor_bindings
            .get(key)
            .context(format!("No constructor found {:?}", key))
            .copied()
    }

    pub fn insert_constructor_binding(&mut self, key: &str, value: TypeIdx) {
        self.constructor_bindings.insert(key.into(), value);
    }

    pub fn remove_constructor_binding(&mut self, key: &str) {
        self.constructor_bindings.remove(key);
    }
}
