use use_constraint::Constraint;

fn main() {
    let constraint = Constraint::<str, _>::new("non-empty", |value| !value.is_empty());
    let evaluation = constraint.evaluate("rustuse");

    assert_eq!(evaluation.label(), "non-empty");
    assert!(evaluation.satisfied());
}
