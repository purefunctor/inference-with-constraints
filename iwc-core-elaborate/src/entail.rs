pub struct Entail<'context> {
    context: &'context mut crate::context::Context,
}

impl<'context> Entail<'context> {
    pub fn new(context: &'context mut crate::context::Context) -> Self {
        Self { context }
    }
}

/*

A new entailment algorithm:

The `ClassEntail` constraint attempts to solve a given assertion with local
and global instances. This is where we start to branch off a bit:

Given the local and global instances:

If there exists a match, and no other dependencies are needed, then we
consider this assertion to be solved. If there are other dependencies to
be solved, then we consider the assertion to be deferred, and schedule
the dependencies to be solved as a `ClassEntail` constraint.

If no match exists, we defer the assertion and consider it failing. If
an instance gets added through some other constraint, we can schedule
the deferred assertion to be queued again.

Given the nuances of other features in the language, what constitutes
as an instance "match" may vary. At least within the realm of functional
dependencies, we consider the following algorithm:

For each local and global instance, perform a unification of the assertion
and instance arguments. Arguments appearing in the domain of a functional
dependency _must_ solve to a concrete type. If not, we defer the constraint
until this is solved. Arguments appearing in the codomain of a functional
do not have this restriction.

On the topic of matching instance heads, the act of matching each argument
can either be done by using the core unification routine, or a custom one
that returns substitutions inline. The challenge with the former though
is non-linearity. Given the following instance:

```hs
instance appendNil :: Append Nil ys ys
```

To start, the free variables would need to be instantiated, just like so:

```hs
Append Nil ?ys ?ys
```

When matching with an assertion, such as the following:

```hs
Append Nil (Cons 0 Nil) ?zs
```

We can see constraints go something along the lines of:

```hs
ClassEntail(Append Nil (Cons 0 Nil) ?zs)

?ys ~ (Cons 0 Nil)
?ys ~ ?zs

ClassEntail(Append Nil (Cons 0 Nil) ?zs)
```

The problem for this approach is that once the second `ClassEntail` constraint
needs to be solved, we need to restore information before resumption. Specifically,
which instance was being matched against, and the instantiated version of the instance.

Here's what we could do though, each assertion being entailed should associate with
a unique index that is then used as the key for storing information between non-linear
calls to entail. We can use this index to store information such as the instances being
tried and the substitutions for each instance.

*/
