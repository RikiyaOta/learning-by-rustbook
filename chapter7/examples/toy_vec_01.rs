use chapter7::ToyVec;

fn main() {
    let mut v = ToyVec::new();
    v.push("Java Finch".to_string());
    v.push("Budgerigar".to_string());

    let e = v.get(1);

    assert_eq!(e, Some(&"Budgerigar".to_string()));
}
