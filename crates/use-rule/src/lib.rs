#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Named reusable rule primitives.

use core::marker::PhantomData;
use use_check::{CheckResult, check};

pub mod prelude;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuleEvaluation<'a> {
    name: &'a str,
    result: CheckResult,
}

impl<'a> RuleEvaluation<'a> {
    #[must_use]
    pub const fn name(&self) -> &'a str {
        self.name
    }

    #[must_use]
    pub const fn result(&self) -> CheckResult {
        self.result
    }

    #[must_use]
    pub const fn passed(&self) -> bool {
        self.result.is_pass()
    }
}

pub struct Rule<T: ?Sized, F> {
    name: &'static str,
    checker: F,
    marker: PhantomData<fn(&T)>,
}

impl<T: ?Sized, F> Rule<T, F>
where
    F: Fn(&T) -> bool,
{
    #[must_use]
    pub const fn new(name: &'static str, checker: F) -> Self {
        Self {
            name,
            checker,
            marker: PhantomData,
        }
    }

    #[must_use]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    #[must_use]
    pub fn evaluate(&self, value: &T) -> RuleEvaluation<'static> {
        RuleEvaluation {
            name: self.name,
            result: check((self.checker)(value)),
        }
    }

    #[must_use]
    pub fn passes(&self, value: &T) -> bool {
        self.evaluate(value).passed()
    }
}

#[cfg(test)]
mod tests {
    use super::Rule;

    #[test]
    fn rules_keep_a_name_and_result() {
        let rule = Rule::<i32, _>::new("positive", |value| *value > 0);
        let evaluation = rule.evaluate(&42);

        assert_eq!(evaluation.name(), "positive");
        assert!(evaluation.passed());
    }
}
