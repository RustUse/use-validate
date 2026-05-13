# use-bound

Inclusive and exclusive bound primitives for `RustUse`.

## Install

```toml
[dependencies]
use-bound = "0.0.1"
```

## Foundation

`use-bound` models lower and upper bounds explicitly, with inclusive and exclusive variants that work with any `PartialOrd` value.

## Example

```rust
use use_bound::{exclusive_minimum, maximum};

let lower = exclusive_minimum(0);
let upper = maximum(10);

assert!(lower.allows(&1));
assert!(upper.allows(&10));
```

## When to use directly

Choose `use-bound` when you only need reusable lower and upper bound primitives.

## Scope

- Bounds stay generic over `PartialOrd` values.
- Higher-level range assembly lives in `use-range`.

## Status

`use-bound` is a pre-1.0 crate with a deliberately small API.
