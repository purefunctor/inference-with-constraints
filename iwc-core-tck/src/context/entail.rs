//! Implements the entailment algorithm.

use iwc_core_ast::ty::{Assertion, TyIdx};

use super::Context;

impl Context {
    // The entailment algorithm involves matching the assertion arguments similar to unification
    // but not entirely the same. To start, only monotypes can appear in instance heads, excluding
    // unification variables; we'll call this the assertion matching algorithm moving forward.
    //
    // Given an assertion, it finds the instances for its class
    //
    // For each instance, attempt to match it with the current assertion.
    //
    // Concrete types get matched like normal, _but_ when the instance argument contains a syntactic
    // variable, we "solve" it to whatever is in the assertion, if it's encountered again in the future
    // we simply resolve the type.
    //
    // Questions:
    //
    // Can we offload assertion matching to the inference constraint solver too?
    //
    // My gut feeling says that we can, and that the algorithm would be something along the lines of
    //
    // 1. Generate a ClassAssertion constraint, which warrants an entailment call
    // 2. The entailment call performs assertion matching, which generates constraints
    // 3. Once control flow is passed back to the inference constraint solver, more constraints
    //    are there to be solved, in particular with binding type variables.
    //
    // Let's say we have a `Item (List a) a` instance, and we want to entail the assertion `Item (List Int) Int`
    //
    // We start with a ClassAssertion(Item, List Int, Int)
    //
    // first argument: List a ~ List Int -> a ~ Int - ClassMatchSolve(a, Int) is pushed
    // second argument: a ~ Int - ClassMatchSolve(a, Int) is _also_ pushed
    //
    // our next set of constraints is now [ClassMatchSolve(a, Int), ClassMatchSolve(a, Int)]
    //
    // we store the information that `a ~ Int`, so when we solve the latter, it resolves and
    // we have a nice time!
    //
    // On the other hand, let's say we have `Item (List Int) Number` which is an oopsie.
    //
    // first argument: List a ~ List Int -> a ~ Int - ClassMatchSolve(a, Int) is pushed
    // second argument: a ~ Number - ClassMatchSolve(a, Number) is pushed
    //
    // By the time the inference constraint solver matches the second argument, it'll fail,
    // however, we don't really want that. What we do want is for it to attempt other instances.
    // How do we communicate that we should try more instances? Well, we could always "track"
    // the `ClassAssertion` at the first entailment call. If any of the ClassX constraints fail
    // we just reinsert the `ClassAssertion` back. We also have to communicate to entail that
    // it should try a different instance! Depending on how instances are stored on the context
    // usually, this can just be `HashMap<ClassName, Vec<Instance>>`, we could probably store
    // the index of the instances that have been tried so far as well.
    //
    // We should also implement a scoping mechanism for solving constraints. For example, if
    // we have the instances `Foo a` and `Bar a`, but the assertions `Foo Int` and `Bar Number`,
    // we should absolutely separate the scopes for each instance. One way to do this is to
    // add a fresh index to each ClassAssertion that serves as a key.
    //
    // So then, we would have the following contraints being solved
    //
    // ClassAssertion { index: usize, assertion: Assertion }
    // MatchSolve { index: usize, variable: SmolStr, ty_idx: TyIdx }
    // MatchDeep { l_variable: SmolStr, r_variable: SmolStr }
    //
    // As for the meta-context, we'd have:
    //
    // assertion_solutions: HashMap<Index, HashMap<SmolStr, TyIdx>>
    // assertion_instance_index: HashMap<Index, usize>
    // assertion_stack: Vec<Index> - this keeps track of when we solve dependencies
    //
    // What constraints modify which:
    //
    // ClassAssertion:
    //
    // Creates a new entry to the assertion_instance_index
    // Pushes an entry to the assertion_stack
    //
    // MatchSolve:
    //
    // Pushes an entry to assertion_solutions

    pub fn match_assertion(
        &mut self,
        instance: Assertion,
        assertion: Assertion,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn match_assertion_argument(&mut self, i_idx: TyIdx, a_idx: TyIdx) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn entail(&mut self, assertion: Assertion) -> anyhow::Result<()> {
        Ok(())
    }
}
