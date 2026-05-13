use use_predicate::{all, any, count, not};

fn main() {
    let predicates: [fn(&i32) -> bool; 2] = [|value| *value > 0, |value| *value % 2 == 0];

    assert!(all(&4, &predicates));
    assert!(any(&3, &predicates));
    assert_eq!(count(&4, &predicates), 2);
    assert!(not(&3, |value| *value < 0));
}
