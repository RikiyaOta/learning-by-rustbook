use super::SortOrder;
use rayon;
use std::cmp::Ordering;

pub fn sort_by<T, F>(x: &mut [T], comparator: &F) -> Result<(), String>
where
    T: Send,
    F: Sync + Fn(&T, &T) -> Ordering,
{
    if x.len().is_power_of_two() {
        do_sort(x, true, comparator);
        Ok(())
    } else {
        Err(format!(
            "The length of x is not a power of two. (x.len(): {})",
            x.len()
        ))
    }
}

pub fn sort<T: Ord + Send>(x: &mut [T], order: &SortOrder) -> Result<(), String> {
    match *order {
        SortOrder::Ascending => sort_by(x, &|a, b| a.cmp(b)),
        SortOrder::Descending => sort_by(x, &|a, b| b.cmp(a)),
    }
}

// 並列に処理するかを決める閾値
const PARALLEL_THRESHOLD: usize = 4096;

fn do_sort<T, F>(x: &mut [T], forward: bool, comparator: &F)
where
    T: Send,
    F: Sync + Fn(&T, &T) -> Ordering,
{
    if x.len() > 1 {
        let mid_point = x.len() / 2;

        // x の分割後の要素数を閾値と比較する
        if mid_point >= PARALLEL_THRESHOLD {
            // もし閾値以上なら、並列にソートする

            // x を事前に分割しておく（所有権の仕組み上、可変参照を同時に実行される2つのクロージャーに渡すことができない。

            // ↓しかしこれはダメ。first の束縛時点で、x に対する可変参照を作っているので、second に可変参照を渡すことができない。
            // let first = &mut x[..mid_point];
            // let second = &mut x[mid_point..];

            // ↓なのでこうする。
            let (first, second) = x.split_at_mut(mid_point);

            rayon::join(
                || do_sort(first, true, comparator),
                || do_sort(second, false, comparator),
            );
        } else {
            // 閾値未満なら順番にソートする
            do_sort(&mut x[..mid_point], true, comparator);
            do_sort(&mut x[mid_point..], false, comparator);
        }

        sub_sort(x, forward, comparator);
    }
}

fn sub_sort<T, F>(x: &mut [T], forward: bool, comparator: &F)
where
    F: Fn(&T, &T) -> Ordering,
{
    if x.len() > 1 {
        compare_and_swap(x, forward, comparator);
        let mid_point = x.len() / 2;
        sub_sort(&mut x[..mid_point], forward, comparator);
        sub_sort(&mut x[mid_point..], forward, comparator);
    }
}

fn compare_and_swap<T, F>(x: &mut [T], forward: bool, comparator: &F)
where
    F: Fn(&T, &T) -> Ordering,
{
    let swap_condition = if forward {
        Ordering::Greater
    } else {
        Ordering::Less
    };

    let mid_point = x.len() / 2;
    for i in 0..mid_point {
        if comparator(&x[i], &x[mid_point + i]) == swap_condition {
            x.swap(i, mid_point + i);
        }
    }
}

// このモジュールは cargo test を実行した時のみコンパイルされる
#[cfg(test)]
mod tests {
    use std::vec;

    use super::{sort, sort_by};
    use crate::utils::{is_sorted_ascending, is_sorted_descending, new_u32_vec};
    use crate::SortOrder::*;

    #[derive(Debug, PartialEq)]
    struct Student {
        first_name: String,
        last_name: String,
        age: u8,
    }

    impl Student {
        fn new(first_name: &str, last_name: &str, age: u8) -> Self {
            Student {
                first_name: first_name.to_string(),
                last_name: last_name.to_string(),
                age,
            }
        }
    }

    #[test]
    fn sort_students_by_age_ascending() {
        let taro = Student::new("Taro", "Yamada", 16);
        let hanako = Student::new("Hanako", "Yamada", 14);
        let kyoko = Student::new("Kyoko", "Hayashi", 15);
        let ryosuke = Student::new("Ryosuke", "Hayashi", 17);

        let mut x = vec![&taro, &hanako, &kyoko, &ryosuke];
        let expected = vec![&hanako, &kyoko, &taro, &ryosuke];

        assert_eq!(sort_by(&mut x, &|a, b| a.age.cmp(&b.age)), Ok(()));

        assert_eq!(x, expected);
    }

    #[test]
    fn sort_students_by_name_ascending() {
        let taro = Student::new("Taro", "Yamada", 16);
        let hanako = Student::new("Hanako", "Yamada", 14);
        let kyoko = Student::new("Kyoko", "Ito", 15);
        let ryosuke = Student::new("Ryosuke", "Hayashi", 17);

        let mut x = vec![&taro, &hanako, &kyoko, &ryosuke];
        let expected = vec![&ryosuke, &kyoko, &hanako, &taro];

        assert_eq!(
            sort_by(&mut x, &|a, b| a
                .last_name
                .cmp(&b.last_name)
                .then_with(|| a.first_name.cmp(&b.first_name))),
            Ok(())
        );

        assert_eq!(x, expected);
    }

    #[test]
    fn sort_u32_ascending() {
        let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];

        assert_eq!(sort(&mut x, &Ascending), Ok(()));

        assert_eq!(x, vec![4, 10, 11, 20, 21, 30, 110, 330]);
    }

    #[test]
    fn sort_u32_descending() {
        let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];

        assert_eq!(sort(&mut x, &Descending), Ok(()));

        assert_eq!(x, vec![330, 110, 30, 21, 20, 11, 10, 4]);
    }

    #[test]
    fn sort_u32_large() {
        {
            // 65536 = 2^16
            let mut x = new_u32_vec(65536);
            assert_eq!(sort(&mut x, &Ascending), Ok(()));
            // ソート結果が正しいことをテスト
            assert!(is_sorted_ascending(&x));
        }
        {
            // 65536 = 2^16
            let mut x = new_u32_vec(65536);
            assert_eq!(sort(&mut x, &Descending), Ok(()));
            // ソート結果が正しいことをテスト
            assert!(is_sorted_descending(&x));
        }
    }

    #[test]
    fn sort_str_ascending() {
        let mut x = vec![
            "Rust",
            "is",
            "fast",
            "and",
            "memory-efficient",
            "with",
            "no",
            "GC",
        ];

        assert_eq!(sort(&mut x, &Ascending), Ok(()));

        assert_eq!(
            x,
            vec![
                "GC",
                "Rust",
                "and",
                "fast",
                "is",
                "memory-efficient",
                "no",
                "with"
            ]
        );
    }

    #[test]
    fn sort_str_descending() {
        let mut x = vec![
            "Rust",
            "is",
            "fast",
            "and",
            "memory-efficient",
            "with",
            "no",
            "GC",
        ];

        assert_eq!(sort(&mut x, &Descending), Ok(()));

        assert_eq!(
            x,
            vec![
                "with",
                "no",
                "memory-efficient",
                "is",
                "fast",
                "and",
                "Rust",
                "GC"
            ]
        );
    }

    #[test]
    fn sort_to_fail() {
        let mut x = vec![10, 30, 11];
        assert!(sort(&mut x, &Ascending).is_err());
    }
}
