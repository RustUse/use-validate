use use_rule::Rule;

#[test]
fn rules_are_named_and_reusable() {
    let rule = Rule::<i32, _>::new("positive", |value| *value > 0);

    assert!(rule.passes(&42));
    assert!(!rule.passes(&-1));
}
