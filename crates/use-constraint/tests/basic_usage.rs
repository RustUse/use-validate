use use_constraint::Constraint;

#[test]
fn constraints_answer_satisfaction_questions() {
    let constraint = Constraint::<str, _>::new("non-empty", |value| !value.is_empty());

    assert!(constraint.is_satisfied_by("rustuse"));
    assert!(!constraint.is_satisfied_by(""));
}
