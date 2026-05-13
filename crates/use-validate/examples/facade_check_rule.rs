use use_validate::{Rule, check};

fn main() {
    let value = 42;
    let result = check(value > 0);
    let rule = Rule::<i32, _>::new("positive", |input| *input > 0);

    assert!(result.is_pass());
    assert!(rule.passes(&value));
}
