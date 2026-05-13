# use-constraint

Lightweight named constraint primitives for `RustUse`.

## Install

```toml
[dependencies]
use-constraint = "0.0.1"
```

## Foundation

`use-constraint` provides a `Constraint<T, F>` wrapper for named checks that answer whether a value satisfies a specific requirement.

## Example

```rust
use use_constraint::Constraint;

let constraint = Constraint::<str, _>::new("non-empty", |value| !value.is_empty());
let evaluation = constraint.evaluate("rustuse");

assert_eq!(evaluation.label(), "non-empty");
assert!(evaluation.satisfied());
```

## When to use directly

Choose `use-constraint` when you want a named `must satisfy` wrapper without a broader validation framework.

## Scope

- Constraints stay generic over a single input value.
- Evaluation results stay small and explicit.
- Error aggregation and reporting layers are out of scope.

## Status

`use-constraint` is a pre-1.0 crate with a deliberately narrow API.
