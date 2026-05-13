#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Pass/fail check primitives.

pub mod prelude;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CheckResult {
    Pass,
    Fail,
}

impl CheckResult {
    #[must_use]
    pub const fn from_bool(condition: bool) -> Self {
        if condition { Self::Pass } else { Self::Fail }
    }

    #[must_use]
    pub const fn is_pass(self) -> bool {
        matches!(self, Self::Pass)
    }

    #[must_use]
    pub const fn is_fail(self) -> bool {
        matches!(self, Self::Fail)
    }
}

#[must_use]
pub const fn check(condition: bool) -> CheckResult {
    CheckResult::from_bool(condition)
}

#[must_use]
pub const fn pass() -> CheckResult {
    CheckResult::Pass
}

#[must_use]
pub const fn fail() -> CheckResult {
    CheckResult::Fail
}

#[cfg(test)]
mod tests {
    use super::{check, fail, pass};

    #[test]
    fn constructors_match_boolean_intent() {
        assert!(check(true).is_pass());
        assert!(pass().is_pass());
        assert!(fail().is_fail());
    }
}
