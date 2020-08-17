//! First tests for new `dissolve` functionality

use derive_getters::Dissolve;

#[derive(Dissolve)]
struct Number {
    num: u64,
}

fn main() {
    let n = Number { num: 64 };
    let number = n.dissolve();
    assert!(number == 64);
}
