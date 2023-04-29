use chapter7::ToyVec;

fn main() {
    let mut v = ToyVec::new();
    v.push("Java Finch".to_string());
    v.push("Budgerigar".to_string());

    let mut iter = v.iter();

    // v.push("Hill Mynah".to_string());
    // -> push が可変の参照を得ようとするが、iter という普遍参照を借用しているものがいるのでコンパイルエラーになる。

    assert_eq!(iter.next(), Some(&"Java Finch".to_string()));

    // この時は、iter が解放されてるので可変参照を取得できる。
    v.push("Canary".to_string());

    for msg in &v {
        println!("{}", msg);
    }
}
