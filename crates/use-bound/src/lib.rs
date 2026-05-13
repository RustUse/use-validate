#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Inclusive and exclusive bound primitives.

pub mod prelude;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LowerBound<T> {
    Inclusive(T),
    Exclusive(T),
}

impl<T: PartialOrd> LowerBound<T> {
    #[must_use]
    pub fn allows(&self, value: &T) -> bool {
        match self {
            Self::Inclusive(bound) => value >= bound,
            Self::Exclusive(bound) => value > bound,
        }
    }

    #[must_use]
    pub const fn value(&self) -> &T {
        match self {
            Self::Inclusive(bound) | Self::Exclusive(bound) => bound,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpperBound<T> {
    Inclusive(T),
    Exclusive(T),
}

impl<T: PartialOrd> UpperBound<T> {
    #[must_use]
    pub fn allows(&self, value: &T) -> bool {
        match self {
            Self::Inclusive(bound) => value <= bound,
            Self::Exclusive(bound) => value < bound,
        }
    }

    #[must_use]
    pub const fn value(&self) -> &T {
        match self {
            Self::Inclusive(bound) | Self::Exclusive(bound) => bound,
        }
    }
}

#[must_use]
pub const fn minimum<T>(value: T) -> LowerBound<T> {
    LowerBound::Inclusive(value)
}

#[must_use]
pub const fn exclusive_minimum<T>(value: T) -> LowerBound<T> {
    LowerBound::Exclusive(value)
}

#[must_use]
pub const fn maximum<T>(value: T) -> UpperBound<T> {
    UpperBound::Inclusive(value)
}

#[must_use]
pub const fn exclusive_maximum<T>(value: T) -> UpperBound<T> {
    UpperBound::Exclusive(value)
}

#[cfg(test)]
mod tests {
    use super::{exclusive_minimum, maximum};

    #[test]
    fn bounds_apply_inclusive_and_exclusive_logic() {
        let lower = exclusive_minimum(0);
        let upper = maximum(10);

        assert!(lower.allows(&1));
        assert!(!lower.allows(&0));
        assert!(upper.allows(&10));
    }
}
