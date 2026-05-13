use use_bound::{maximum, minimum};
use use_range::RangeConstraint;

#[test]
fn range_constraint_applies_bound_checks() -> Result<(), use_range::RangeError> {
    let range = RangeConstraint::new(Some(minimum(1)), Some(maximum(10)))?;

    assert!(range.contains(&5));
    assert!(!range.contains(&11));

    Ok(())
}
