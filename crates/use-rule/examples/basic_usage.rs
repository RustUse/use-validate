use use_rule::Rule;

fn main() {
    let rule = Rule::<i32, _>::new("positive", |value| *value > 0);
    let evaluation = rule.evaluate(&42);

    assert_eq!(evaluation.name(), "positive");
    assert!(evaluation.passed());
}
