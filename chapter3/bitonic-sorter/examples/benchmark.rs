use num_cpus;

use bitonic_sorter::SortOrder;

use bitonic_sorter::fourth::sort as par_sort;
use bitonic_sorter::third::sort as seq_sort;
use bitonic_sorter::utils::{is_sorted_ascending, new_u32_vec};

use std::str::FromStr;
use std::time::Instant;
use std::{env, f64};

fn main() {
    // 1つ目のコマンドライン引数を文字列として取得
    if let Some(n) = env::args().nth(1) {
        let bits = u32::from_str(&n).expect("error parsing argument");
        // 順次ソート, 並列ソート実行
        run_sorts(bits);
    } else {
        eprintln!(
            "Usage {} <number of elements in bits>",
            env::args().nth(0).unwrap()
        );

        std::process::exit(1);
    }
}

fn run_sorts(bits: u32) {
    // 指定されたビット数からデータの要素数を求める
    let len = 2.0_f64.powi(bits as i32) as usize;

    println!(
        "sorting {} integers ({:.1} MB)",
        len,
        (len * std::mem::size_of::<u32>()) as f64 / 1024.0 / 1024.0
    );

    println!(
        "cpu info: {} physical cores, {} logical cores",
        num_cpus::get_physical(),
        num_cpus::get()
    );

    let seq_duration = timed_sort(&seq_sort, len, "seq_sort");
    let par_duration = timed_sort(&par_sort, len, "par_sort");

    println!("speed up: {:.2}x", seq_duration / par_duration);
}

fn timed_sort<F>(sorter: &F, len: usize, name: &str) -> f64
where
    F: Fn(&mut [u32], &SortOrder) -> Result<(), String>,
{
    // 要素数 len の u32 型ベクタを生成する
    let mut x = new_u32_vec(len);

    let start = Instant::now();
    sorter(&mut x, &SortOrder::Ascending).expect("Failed to sort: ");
    // ソートにかかった時間を記録
    let dur = start.elapsed();

    // ソートした要素数をかかった時間（sec）を表示する
    let nano_secs = dur.subsec_nanos() as f64 + dur.as_secs() as f64 * 1e9_f64;
    println!(
        "{}: sorted {} integers in {} seconds",
        name,
        len,
        nano_secs / 1e9
    );

    // ソート結果が正しいか検証
    assert!(is_sorted_ascending(&x));

    nano_secs
}
