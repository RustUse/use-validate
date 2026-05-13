# use-check

Pass/fail check primitives for `RustUse`.

## Install

```toml
[dependencies]
use-check = "0.0.1"
```

## Foundation

`use-check` provides the smallest shared validation vocabulary in this workspace: a `CheckResult` enum plus tiny helper constructors.

## Example

```rust
use use_check::{check, fail, pass};

assert!(check(true).is_pass());
assert!(pass().is_pass());
assert!(fail().is_fail());
```

## When to use directly

Choose `use-check` when a lightweight pass/fail result is the only validation primitive you need.

## Scope

- Results stay boolean and explicit.
- Aggregation, messages, and reporting layers are out of scope.

## Status

`use-check` is a pre-1.0 crate with a deliberately tiny API.
