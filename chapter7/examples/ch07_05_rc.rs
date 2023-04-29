/*
 * Learn Rc type
 */

use std::rc::Rc;

// Parent, Child struct を消してしまったようなので（泣）、
// ここで定義し直します。

#[derive(Debug)]
struct Parent(usize, Child, Child);

#[derive(Debug)]
struct Child(usize);

fn main() {
    let mut rc1 = Rc::new(Child(1));

    // strong_count で、この Child の3章カウントが得られる
    println!("(a) count: {}, rc1: {:?}", Rc::strong_count(&rc1), rc1);

    {
        // 参照カウントが1つ増える
        let rc2 = Rc::clone(&rc1);
        println!(
            "(b) count: {}, rc1: {:?}, rc2: {:?}",
            Rc::strong_count(&rc1),
            rc1,
            rc2
        );
    } // rc2 がスコープを抜けるので、参照カウントが1つ減る

    println!("(c) count: {}, rc1: {:?}", Rc::strong_count(&rc1), rc1);

    // 参照カウントが１の時は可変の参照が得られる。
    if let Some(child) = Rc::get_mut(&mut rc1) {
        child.0 += 1;
    }

    println!("(d) count: {}, rc1: {:?}", Rc::strong_count(&rc1), rc1);

    // Rc::downgrade で Weak ポイントが得られる。
    let weak = Rc::downgrade(&rc1);

    println!(
        "(e) count: {}, rc1: {:?}, weak: {:?}",
        Rc::strong_count(&rc1), // 参照カウンタは1。Weak ポインタはカウントされない。
        rc1,
        weak
    );

    // Weak を Rc にアップグレードすると Child にアクセスできる。
    if let Some(rc3) = weak.upgrade() {
        println!(
            "(f) count: {}, rc1: {:?}, rc3: {:?}",
            Rc::strong_count(&rc1),
            rc1,
            rc3,
        );
    }

    // rc1 をドロップする（スコープを抜けたのと同じ）
    // 3章カウントが０になり、Child は破棄される。
    std::mem::drop(rc1);
    println!("(g) count: 0, weak.upgrade(): {:?}", weak.upgrade());
}
