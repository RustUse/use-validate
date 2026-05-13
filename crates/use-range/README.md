# use-range

Comparable range validation primitives for `RustUse`.

## Install

```toml
[dependencies]
use-range = "0.0.1"
```

## Foundation

`use-range` composes optional lower and upper bounds into a reusable `RangeConstraint<T>` that can validate comparable values.

## Example

```rust
use use_bound::{maximum, minimum};
use use_range::RangeConstraint;

let range = RangeConstraint::new(Some(minimum(1)), Some(maximum(10)))?;

assert!(range.contains(&5));
assert!(!range.contains(&11));
# Ok::<(), use_range::RangeError>(())
```

## When to use directly

Choose `use-range` when you need one reusable range primitive without a broader validation facade.

## Scope

- Ranges stay generic over `PartialOrd` values.
- Bound construction lives in `use-bound`.
- Domain-specific error reporting is out of scope.

## Status

`use-range` is a pre-1.0 crate with a deliberately small API.
