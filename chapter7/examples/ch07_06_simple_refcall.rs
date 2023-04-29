struct A {
    c: char,
    s: String,
}

fn main() {
    let a = A {
        c: 'a',
        s: "alex".to_string(),
    };
    let r = &a; // Create the immutable reference
    r.s.push('a'); // 普遍参照経由ではコンパイルエラーになる。
}
