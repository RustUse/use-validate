use use_check::{check, fail, pass};

fn main() {
    assert!(check(true).is_pass());
    assert!(pass().is_pass());
    assert!(fail().is_fail());
}
