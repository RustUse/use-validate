#[cfg(all(
    feature = "check",
    feature = "rule",
    feature = "range",
    feature = "bound",
    feature = "constraint",
    feature = "predicate"
))]
#[test]
fn facade_exposes_all_namespace_features() {
    use use_validate::{
        bound as _, check as _, constraint as _, predicate as _, range as _, rule as _,
    };
}

#[cfg(all(
    feature = "check",
    not(feature = "rule"),
    not(feature = "range"),
    not(feature = "bound"),
    not(feature = "constraint"),
    not(feature = "predicate")
))]
#[test]
fn facade_supports_check_only() {
    let result = use_validate::check(true);
    assert!(result.is_pass());
}
