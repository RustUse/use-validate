use use_check::{check, fail, pass};

#[test]
fn check_results_are_explicit() {
    assert!(check(true).is_pass());
    assert!(pass().is_pass());
    assert!(fail().is_fail());
}
