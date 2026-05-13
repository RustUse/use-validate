#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Facade for `RustUse` validation primitives.

#[cfg(feature = "bound")]
pub use use_bound as bound;

#[cfg(feature = "bound")]
pub use use_bound::{
    LowerBound, UpperBound, exclusive_maximum, exclusive_minimum, maximum, minimum,
};

#[cfg(feature = "check")]
pub use use_check as check;

#[cfg(feature = "check")]
pub use use_check::{CheckResult, check, fail, pass};

#[cfg(feature = "constraint")]
pub use use_constraint as constraint;

#[cfg(feature = "constraint")]
pub use use_constraint::{Constraint, ConstraintEvaluation};

#[cfg(feature = "predicate")]
pub use use_predicate as predicate;

#[cfg(feature = "predicate")]
pub use use_predicate::{all, any, count, not};

#[cfg(feature = "range")]
pub use use_range as range;

#[cfg(feature = "range")]
pub use use_range::{RangeConstraint, RangeError};

#[cfg(feature = "rule")]
pub use use_rule as rule;

#[cfg(feature = "rule")]
pub use use_rule::{Rule, RuleEvaluation};

pub mod prelude;
