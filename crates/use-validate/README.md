# use-validate

Feature-gated facade for the focused `RustUse` validation crates.

## Install

```toml
[dependencies]
use-validate = { version = "0.0.1", default-features = false, features = ["check", "rule"] }
```

## Foundation

`use-validate` re-exports the focused validation crates in this workspace behind opt-in features. It keeps the facade thin: direct root re-exports for common types and functions, plus nested modules that mirror the concrete crate boundaries.

## Example

```rust
# #[cfg(all(feature = "check", feature = "rule"))]
# fn main() {
use use_validate::{Rule, check};

let value = 42;
let result = check(value > 0);
let rule = Rule::<i32, _>::new("positive", |input| *input > 0);

assert!(result.is_pass());
assert!(rule.passes(&value));
# }
# #[cfg(not(all(feature = "check", feature = "rule")))]
# fn main() {}
```

## When to use directly

Choose `use-validate` when you want one dependency and one import surface. Prefer the focused crates directly when you only need one validation domain.

## Scope

- The facade stays close to the focused crate APIs.
- Feature flags map directly to the focused crates in this workspace.
- Full validation frameworks and error aggregation layers are out of scope.

## Status

`use-validate` is a pre-1.0 crate with a deliberately small facade over focused validation primitives.
