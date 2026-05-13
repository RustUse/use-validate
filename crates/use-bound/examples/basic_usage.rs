use use_bound::{exclusive_minimum, maximum};

fn main() {
    let lower = exclusive_minimum(0);
    let upper = maximum(10);

    assert!(lower.allows(&1));
    assert!(upper.allows(&10));
}
