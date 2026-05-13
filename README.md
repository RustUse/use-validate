# RustUse/use-validate

Composable validation checks, rules, constraints, predicates, and validation results for Rust.

## Workspace crates

| Crate            | Path                     | Purpose                                                 |
| ---------------- | ------------------------ | ------------------------------------------------------- |
| `use-validate`   | `crates/use-validate/`   | Feature-gated facade over the focused validation crates |
| `use-check`      | `crates/use-check/`      | Pass/fail check primitives                              |
| `use-rule`       | `crates/use-rule/`       | Named reusable rule primitives                          |
| `use-range`      | `crates/use-range/`      | Comparable range validation helpers                     |
| `use-bound`      | `crates/use-bound/`      | Inclusive and exclusive bound primitives                |
| `use-constraint` | `crates/use-constraint/` | Lightweight named constraint wrappers                   |
| `use-predicate`  | `crates/use-predicate/`  | Small predicate composition helpers                     |

## Installation

Use the workspace directly or depend on a Git revision until the first crates.io release is published.

## Basic usage

```rust
use use_check::check;
use use_rule::Rule;

let value = 42;
let result = check(value > 0);
let rule = Rule::<i32, _>::new("positive", |input| *input > 0);

assert!(result.is_pass());
assert!(rule.passes(&value));
```

## License

Licensed under either of the following, at your option:

- MIT license: https://github.com/RustUse/.github/blob/main/LICENSE-MIT
- Apache License, Version 2.0: https://github.com/RustUse/.github/blob/main/LICENSE-APACHE

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for the local validation flow and repository policy.
