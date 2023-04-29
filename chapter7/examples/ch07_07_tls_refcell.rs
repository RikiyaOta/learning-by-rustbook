/*
 * TLS = Thread Local Storage
 */

use std::cell::RefCell;
use std::collections::HashSet;

fn main() {
    thread_local!(
       // TLS に変数 RABBITS を作成する
       // threal_local マクロは mut キーワードをサポートしない

        static RABBITS: RefCell<HashSet<&'static str>> = {
            let rb = ["ロップイヤー", "ダッチ"].iter().cloned().collect();
            RefCell::new(rb)
        }
    );

    // TLS においた値にアクセスするには with を使う
    // main スレッドの RABBITS が得られる。
    RABBITS.with(|rb| {
        assert!(rb.borrow().contains("ロップイヤー"));
        rb.borrow_mut().insert("ネザーランド・ドワーフ");
    });

    std::thread::spawn(|| 
       // 別スレッドで insert する
        RABBITS.with(|rb| rb.borrow_mut().insert("ドワーフホト")
    ))
    .join()
    .expect("Thread error"); // スレッドの終了を待つ。

    // main スレッドの RABBITS にアクセスする
    RABBITS.with(|rb| {
        assert!(rb.borrow().contains("ネザーランド・ドワーフ"));

        // RABBITS はスレッドごとに持つので、別スレッドで追加した要素はここにはない
        assert!(!rb.borrow().contains("ドワーフホト"));
    })
}
