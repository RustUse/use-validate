#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Small predicate composition helpers.

pub mod prelude;

#[must_use]
pub fn all<T>(value: &T, predicates: &[fn(&T) -> bool]) -> bool {
    predicates.iter().all(|predicate| predicate(value))
}

#[must_use]
pub fn any<T>(value: &T, predicates: &[fn(&T) -> bool]) -> bool {
    predicates.iter().any(|predicate| predicate(value))
}

#[must_use]
pub fn not<T>(value: &T, predicate: fn(&T) -> bool) -> bool {
    !predicate(value)
}

#[must_use]
pub fn count<T>(value: &T, predicates: &[fn(&T) -> bool]) -> usize {
    predicates
        .iter()
        .filter(|predicate| predicate(value))
        .count()
}

#[cfg(test)]
mod tests {
    use super::{all, any, count, not};

    #[test]
    fn predicate_helpers_compose_cleanly() {
        let predicates: [fn(&i32) -> bool; 2] = [|value| *value > 0, |value| *value % 2 == 0];

        assert!(all(&4, &predicates));
        assert!(any(&3, &predicates));
        assert_eq!(count(&4, &predicates), 2);
        assert!(not(&3, |value| *value < 0));
    }
}
