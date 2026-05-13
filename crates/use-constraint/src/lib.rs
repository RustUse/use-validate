#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Lightweight named constraint primitives.

use core::marker::PhantomData;
use use_check::{CheckResult, check};

pub mod prelude;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstraintEvaluation<'a> {
    label: &'a str,
    result: CheckResult,
}

impl<'a> ConstraintEvaluation<'a> {
    #[must_use]
    pub const fn label(&self) -> &'a str {
        self.label
    }

    #[must_use]
    pub const fn result(&self) -> CheckResult {
        self.result
    }

    #[must_use]
    pub const fn satisfied(&self) -> bool {
        self.result.is_pass()
    }
}

pub struct Constraint<T: ?Sized, F> {
    label: &'static str,
    evaluator: F,
    marker: PhantomData<fn(&T)>,
}

impl<T: ?Sized, F> Constraint<T, F>
where
    F: Fn(&T) -> bool,
{
    #[must_use]
    pub const fn new(label: &'static str, evaluator: F) -> Self {
        Self {
            label,
            evaluator,
            marker: PhantomData,
        }
    }

    #[must_use]
    pub const fn label(&self) -> &'static str {
        self.label
    }

    #[must_use]
    pub fn evaluate(&self, value: &T) -> ConstraintEvaluation<'static> {
        ConstraintEvaluation {
            label: self.label,
            result: check((self.evaluator)(value)),
        }
    }

    #[must_use]
    pub fn is_satisfied_by(&self, value: &T) -> bool {
        self.evaluate(value).satisfied()
    }
}

#[cfg(test)]
mod tests {
    use super::Constraint;

    #[test]
    fn constraints_report_satisfaction() {
        let constraint = Constraint::<str, _>::new("non-empty", |value| !value.is_empty());
        let evaluation = constraint.evaluate("rustuse");

        assert_eq!(evaluation.label(), "non-empty");
        assert!(evaluation.satisfied());
    }
}
