pub mod context;
pub mod infer;
pub mod instantiate;
pub mod solve;
pub mod unify;

#[cfg(test)]
mod tests {
    use crate::context::Context;

    #[test]
    fn api_construction() {
        let _ = Context::default();
    }
}
