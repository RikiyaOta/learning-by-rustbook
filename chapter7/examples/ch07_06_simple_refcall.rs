use std::cell::RefCell;

//struct A {
//    c: char,
//    s: String,
//}

struct B {
    c: char,
    s: RefCell<String>, // String を RefCell で包む
}

fn main() {
    //let a = A {
    //    c: 'a',
    //    s: "alex".to_string(),
    //};
    //let r = &a; // Create the immutable reference
    //r.s.push('a'); // 普遍参照経由ではコンパイルエラーになる。

    let b = B {
        c: 'a',
        s: RefCell::new("alex".to_string()),
    };
    let rb = &b;
    rb.s.borrow_mut().push('a'); // フィールドsのデータに対する可変の参照をとる。

    {
        let rbs = b.s.borrow(); // 普遍の参照をとる。
        assert_eq!(&*rbs, "alexa");

        // RefCell では他の参照が有効な間に他の参照を取ろうとすると実行時にパニックになる。
        b.s.borrow_mut(); // この時点で、普遍の参照 rbs がまだ有効

        // try_borrow_mut ならパニックせず Err を返してくれる。
        assert!(b.s.try_borrow_mut().is_err()); // Err が返る
    } // rbs はここでスコープを抜ける。

    assert!(b.s.try_borrow_mut().is_ok());
}
