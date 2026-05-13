use use_bound::{exclusive_minimum, maximum};

#[test]
fn bounds_are_explicit_about_inclusivity() {
    let lower = exclusive_minimum(0);
    let upper = maximum(10);

    assert!(lower.allows(&1));
    assert!(!lower.allows(&0));
    assert!(upper.allows(&10));
}
