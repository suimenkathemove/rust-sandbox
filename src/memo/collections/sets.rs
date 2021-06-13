use std::collections::HashSet;

fn main() {
    // セットは、ある値が含まれているかの判定を高速に行うことができるコレクション

    // セットの実装は、キーと値のペアのうち、キーだけを持つマップのようなもの
    // HashSet<T>の実装はHashMap<T, ()>、BTreeSet<T>の実装はBTreeMap<T, ()>をラップしただけのもの

    let mut hs: HashSet<i32> = HashSet::new();

    assert!(hs.insert(0));
    assert!(!hs.insert(0));

    assert!(hs.remove(&0));
    assert!(!hs.remove(&0));

    {
        // イテレート

        // セット内の要素に対する、mut参照での繰り返し処理はサポートされていない
    }

    {
        // 値が等しいが別のものの場合

        let mut hs: HashSet<i32> = HashSet::new();

        hs.insert(0);
        assert_eq!(hs.get(&0), Some(&0));
        assert_eq!(hs.take(&0), Some(0));

        hs.insert(0);
        // 等しい値が既に入っていた場合には、それを置き換えて古い値を返す
        assert_eq!(hs.replace(0), Some(0));
        assert_eq!(hs.replace(1), None);
    }

    {
        // 全体に対する演算

        {
            // intersection

            let set1 = (0..3).collect::<HashSet<i32>>();
            let set2 = (1..4).collect::<HashSet<i32>>();

            for i in set1.intersection(&set2) {
                println!("{}", i);
            }

            for i in &set1 & &set2 {
                println!("{}", i);
            }
        }

        {
            // union
        }

        {
            // difference

            // set1.difference(&set2)
            // &set1 - &set2
        }

        {
            // symmetric_difference

            // set1.symmetric_difference(&set2)
            // &set1 ^ &set2
        }

        // 集合間の関係をテストするためのメソッド

        {
            // is_disjoint

            // 積集合が空集合の場合に真を返す
        }

        {
            // is_subset

            // 部分集合の場合に真を返す
        }

        {
            // is_superset

            // 上位集合の場合に真を返す
        }
    }
}
