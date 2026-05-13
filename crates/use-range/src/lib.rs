#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Comparable range validation helpers.

use core::{cmp::Ordering, fmt};
use use_bound::{LowerBound, UpperBound};

pub mod prelude;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RangeError {
    InvertedBounds,
}

impl fmt::Display for RangeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvertedBounds => formatter.write_str("lower bound cannot exceed upper bound"),
        }
    }
}

impl std::error::Error for RangeError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RangeConstraint<T> {
    lower: Option<LowerBound<T>>,
    upper: Option<UpperBound<T>>,
}

impl<T: PartialOrd> RangeConstraint<T> {
    /// Creates a reusable range from optional lower and upper bounds.
    ///
    /// # Errors
    ///
    /// Returns [`RangeError::InvertedBounds`] when the provided bounds describe an empty or
    /// inverted range.
    pub fn new(
        lower: Option<LowerBound<T>>,
        upper: Option<UpperBound<T>>,
    ) -> Result<Self, RangeError> {
        if let (Some(lower_bound), Some(upper_bound)) = (&lower, &upper) {
            match lower_bound.value().partial_cmp(upper_bound.value()) {
                Some(Ordering::Greater) | None => return Err(RangeError::InvertedBounds),
                Some(Ordering::Equal)
                    if matches!(lower_bound, LowerBound::Exclusive(_))
                        || matches!(upper_bound, UpperBound::Exclusive(_)) =>
                {
                    return Err(RangeError::InvertedBounds);
                },
                Some(Ordering::Less | Ordering::Equal) => {},
            }
        }

        Ok(Self { lower, upper })
    }

    #[must_use]
    pub fn contains(&self, value: &T) -> bool {
        self.lower
            .as_ref()
            .is_none_or(|lower_bound| lower_bound.allows(value))
            && self
                .upper
                .as_ref()
                .is_none_or(|upper_bound| upper_bound.allows(value))
    }
}

#[cfg(test)]
mod tests {
    use super::{RangeConstraint, RangeError};
    use use_bound::{exclusive_maximum, exclusive_minimum, maximum, minimum};

    #[test]
    fn range_contains_values_inside_the_bounds() -> Result<(), RangeError> {
        let range = RangeConstraint::new(Some(minimum(1)), Some(maximum(10)))?;
        assert!(range.contains(&5));
        assert!(!range.contains(&11));
        Ok(())
    }

    #[test]
    fn range_rejects_empty_exclusive_intervals() {
        assert_eq!(
            RangeConstraint::new(Some(exclusive_minimum(1)), Some(exclusive_maximum(1))),
            Err(RangeError::InvertedBounds)
        );
    }
}
