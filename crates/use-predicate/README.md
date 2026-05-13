# use-predicate

Small predicate composition helpers for `RustUse`.

## Install

```toml
[dependencies]
use-predicate = "0.0.1"
```

## Foundation

`use-predicate` provides tiny helpers for running multiple predicates against a single value: `all`, `any`, `not`, and `count`.

## Example

```rust
use use_predicate::{all, any, count, not};

let predicates: [fn(&i32) -> bool; 2] = [|value| *value > 0, |value| *value % 2 == 0];

assert!(all(&4, &predicates));
assert!(any(&3, &predicates));
assert_eq!(count(&4, &predicates), 2);
assert!(not(&3, |value| *value < 0));
```

## When to use directly

Choose `use-predicate` when predicate composition is the only validation primitive you need.

## Scope

- Helpers stay value-first and function-pointer based.
- Rules, constraints, and result types live in adjacent crates.

## Status

`use-predicate` is a pre-1.0 crate with a deliberately small API.
