use std::vec;

fn main() {
    let v = vec!["I", "love", "Rust!"]
        .into_iter()
        .map(|s| s.len())
        .collect::<Vec<_>>();
    assert_eq!(v, vec![1, 4, 5]);
}
