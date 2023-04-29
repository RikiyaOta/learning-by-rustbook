use std::collections::HashSet;
use std::error::Error;
use std::sync::{Arc, RwLock};

fn main() -> Result<(), Box<dyn Error>> {
    let dogs: HashSet<_> = ["柴", "トイプードル"].iter().cloned().collect();

    // HashSet を可変にするために RwLock で包み、スレッド間で共有するためにArcで包む
    let dogs = Arc::new(RwLock::new(dogs));

    fn stringify(x: impl ToString) -> String {
        x.to_string()
    }

    {
        // read lock を取得する
        let ds = dogs.read().map_err(stringify)?;
        assert!(ds.contains("柴"));
        assert!(ds.contains("トイプードル"));
    } // ds がスコープをはずれロックが解除される。

    // write lock を取得して要素を追加する。
    dogs.write().map_err(stringify)?.insert("ブル・テリア");

    let dogs1 = Arc::clone(&dogs);
    std::thread::spawn(move ||
        // 別のスレッドで write ロックを取得し、HashSet に要素を追加する
        dogs1.write().map(|mut ds| ds.insert("コーギー")).map_err(stringify))
    .join()
    .expect("Thread error")?;

    assert!(dogs.read().map_err(stringify)?.contains("ブル・テリア"));
    assert!(dogs.read().map_err(stringify)?.contains("コーギー"));

    Ok(())
}
