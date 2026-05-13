# use-rule

Named reusable rule primitives for `RustUse`.

## Install

```toml
[dependencies]
use-rule = "0.0.1"
```

## Foundation

`use-rule` provides a `Rule<T, F>` type for reusable named checks plus a lightweight `RuleEvaluation` result.

## Example

```rust
use use_rule::Rule;

let rule = Rule::<i32, _>::new("positive", |value| *value > 0);
let evaluation = rule.evaluate(&42);

assert_eq!(evaluation.name(), "positive");
assert!(evaluation.passed());
```

## When to use directly

Choose `use-rule` when you want a named reusable check without a broader framework.

## Scope

- Rules stay generic over a single input value.
- Evaluation results stay small and explicit.
- Error aggregation and reporting layers are out of scope.

## Status

`use-rule` is a pre-1.0 crate with a deliberately narrow API.
