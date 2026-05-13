#[cfg(all(feature = "check", feature = "rule", feature = "predicate"))]
#[test]
fn facade_reexports_core_validation_workflow() {
    use use_validate::prelude::{Rule, all, check};

    let predicates: [fn(&i32) -> bool; 2] = [|value| *value > 0, |value| *value % 2 == 0];
    let result = check(true);
    let rule = Rule::<i32, _>::new("positive", |value| *value > 0);

    assert!(result.is_pass());
    assert!(rule.passes(&4));
    assert!(all(&4, &predicates));
}
