fn main() {
    // イテレータをベクタに変換
    let _v: Vec<i32> = (0..5).collect();

    // 必要とする要素数が事前に分かっているなら、Vec::with_capacityを用いると、要素を追加してもバッファの再確保は生じない
    // vec!マクロは想定よりも多くの要素が必要になったら、自動的にバッファが拡張される

    let v: Vec<i32> = Vec::with_capacity(2);
    assert_eq!(v.len(), 0);
    assert_eq!(v.capacity(), 2);

    // HashSet => Vec

    use std::collections::HashSet;

    let mut s = HashSet::new();
    s.insert(0);

    let _v = s.iter().collect::<Vec<_>>();

    {
        // 要素へのアクセス

        let v: Vec<i32> = (0..3).collect();
        // slice to vector
        let _v2 = v[0..3].to_vec();

        // ベクタの長さとインデックスの型はusize

        // スライスのメソッドは、配列やベクタでも使える

        let mut v: Vec<i32> = (0..3).collect();

        let _first = v.first();
        let _first_mut = v.first_mut();
        let _last = v.last();
        let _last_mut = v.last_mut();
        let _get = v.get(0);
        let _get_mut = v.get_mut(0);

        // 値として返すと移動することになるため、要素にアクセスするメソッドは要素への参照を返す
    }

    {
        // ベクタの伸長と縮小

        // 容量は、通常はVecが自動的に管理してくれる
        // より大きい領域が必要になれば、自動的に大きいバッファを確保し、要素をそこにコピーする

        {
            // 容量を明示的に管理するメソッド

            let mut v: Vec<i32> = Vec::with_capacity(3);
            assert_eq!(v.capacity(), 3);
            assert_eq!(v.len(), 0);

            v.push(0);
            assert_eq!(v.capacity(), 3);
            assert_eq!(v.len(), 1);

            // n個の要素を追加できる容量を確保する
            v.reserve_exact(4);
            assert_eq!(v.capacity(), 5);
            assert_eq!(v.len(), 1);

            // 少なくともn個の要素を追加できる容量を確保する
            v.reserve(5);

            v.shrink_to_fit();
            assert_eq!(v.capacity(), 1);
            assert_eq!(v.len(), 1);
        }

        {
            // 値を移動によって出し入れする

            let mut v: Vec<i32> = (0..3).collect();

            v.insert(1, 3);
            assert_eq!(v, vec![0, 3, 1, 2]);

            assert_eq!(v.remove(1), 3);
            assert_eq!(v, vec![0, 1, 2]);

            // この操作はベクタが長いほど遅くなる
            // 多用するのであれば、VecDequeを使うことを考える
        }

        {
            // 長さを特定の値に変更する

            {
                // resize

                let mut v: Vec<i32> = (0..3).collect();

                v.resize(5, 3);
                assert_eq!(v, vec![0, 1, 2, 3, 3]);

                v.resize(2, 0);
                assert_eq!(v, vec![0, 1]);
            }

            {
                // truncate

                let mut v: Vec<i32> = (0..3).collect();

                v.truncate(2);
                assert_eq!(v, vec![0, 1]);

                v.truncate(3);
                assert_eq!(v, vec![0, 1]);
            }

            {
                // clear

                let mut v: Vec<i32> = (0..3).collect();

                v.clear();
                assert_eq!(v, vec![]);
            }
        }

        {
            // 複数の値を一度に追加、削除する

            {
                // extend

                let mut v1: Vec<i32> = (0..3).collect();
                let v2: Vec<i32> = (3..6).collect();

                v1.extend(&v2);

                assert_eq!(v1, vec![0, 1, 2, 3, 4, 5]);
                assert_eq!(v2, vec![3, 4, 5]);
            }

            {
                // split_off

                // truncateと似ているが、vecの末尾から削除された値を格納したVec<T>を返す

                let mut v: Vec<i32> = (0..3).collect();
                assert_eq!(v.split_off(1), vec![1, 2]);
            }

            {
                // append

                // 引数の要素をすべて移動させる

                let mut v1: Vec<i32> = (0..3).collect();
                let mut v2: Vec<i32> = (3..6).collect();

                v1.append(&mut v2);

                assert_eq!(v1, vec![0, 1, 2, 3, 4, 5]);
                assert_eq!(v2, vec![]);
            }

            {
                // drain

                // vecからvec[range]の範囲を削除して、削除した要素に対するイテレータを返す

                let mut v: Vec<i32> = (0..3).collect();
                assert_eq!(v.drain(0..2).collect::<Vec<_>>(), vec![0, 1]);
            }

            {
                // retain

                let mut v: Vec<i32> = (0..3).collect();
                let f = |&i: &i32| i > 1;

                v.retain(f);
                assert_eq!(v, vec![2]);

                // 性能を除けば以下と同じ
                let v2 = v.into_iter().filter(f).collect::<Vec<i32>>();
                assert_eq!(v2, vec![2]);
            }

            {
                // dedup

                let mut v: Vec<i32> = vec![0, 0, 1, 1, 2, 1, 0];
                v.dedup();
                assert_eq!(v, vec![0, 1, 2, 1, 0]);

                // 要素の重複を排除したい場合

                // ・dedupを呼ぶ前にベクタをソートする
                // ・データをHashSetなどのセットに入れる
                // ・元の状態を維持したい場合はretainを使う

                let mut v: Vec<i32> = vec![0, 0, 1, 1, 2, 1, 0];
                let mut s = HashSet::new();
                v.retain(|i| s.insert(*i));
                assert_eq!(v, vec![0, 1, 2]);
            }

            {
                // dedup_by

                // dedupとほぼ同じだが、2つの要素が等しいかどうかを判断する際に、same(&mut e1, &mut e2)という形の関数もしくはクロージャを用いる
            }

            {
                // dedup_by_key

                // dedupとほぼ同じだが、key(&mut e1) == key(&mut e2)で判断する
            }
        }

        // resizeだけが値のクローンを行い、他のメソッドは移動する
    }

    {
        // 連結

        assert_eq!([[0], [1]].concat(), vec![0, 1]);

        assert_eq!([[0], [1]].join(&2), vec![0, 2, 1]);
    }

    {
        // 分割

        // 配列、スライス、ベクタの、2つ以上の部分から変更可能参照を借用するメソッドがある

        {
            // iter, iter_mut

            let mut v: Vec<i32> = (0..3).collect();
            v.iter();
            v.iter_mut();
        }

        {
            // split_at, split_at_mut

            let v: Vec<i32> = (0..3).collect();
            let (l, r) = v.split_at(1);
            assert_eq!(l, vec![0]);
            assert_eq!(r, vec![1, 2]);

            // split_at(index)は&slice[..index], &slice[index..]と等価である
        }

        {
            // split_first, split_first_mut, split_last, split_last_mut

            let v: Vec<i32> = (0..3).collect();
            let (first, rest) = v.split_first().unwrap();
            assert_eq!(first, &0);
            assert_eq!(rest, vec![1, 2]);
        }

        {
            // split, split_mut

            // サブスライスを生成するイテレータを返す

            let v: Vec<i32> = (0..3).collect();
            assert_eq!(
                v.split(|&i| i == 1).into_iter().collect::<Vec<_>>(),
                vec![[0], [2]]
            );

            // セパレータはサブスライスに含まれない

            // 出力には最低でも1つのサブスライスが含まれ、1つのセパレータごとに1つのサブスライスが追加される

            // セパレータが連続している場合や、最後のアイテムがセパレータであった場合には、結果に空のサブスライスが含まれる
        }

        {
            // splitn, splitn_mut, rsplitn, rsplitn_mut

            // splitとほぼ同じだが、最大でもn個のサブスライスしか作らない
        }

        {
            // chunks, chunks_mut

            let v: Vec<i32> = (0..4).collect();
            assert_eq!(v.chunks(2).collect::<Vec<_>>(), vec![[0, 1], [2, 3]]);
        }

        {
            // windows

            let v: Vec<i32> = (0..4).collect();
            assert_eq!(
                v.windows(2).collect::<Vec<_>>(),
                vec![[0, 1], [1, 2], [2, 3]]
            );
        }
    }

    {
        // 入れ替え

        {
            // swap

            let mut v: Vec<i32> = (0..3).collect();
            v.swap(1, 2);
            assert_eq!(v, vec![0, 2, 1]);
        }

        {
            // swap_remove

            let mut v: Vec<i32> = (0..4).collect();
            v.swap_remove(1);
            assert_eq!(v, vec![0, 3, 2]);

            // ベクタに残される要素の順番を気にしないで良い場合は有用
        }
    }

    {
        // ソートと検索

        struct User {
            name: String,
            age: i32,
        }

        {
            // sort

            let mut v: Vec<i32> = (0..3).rev().collect();
            assert_eq!(v, vec![2, 1, 0]);
            v.sort();
            assert_eq!(v, vec![0, 1, 2]);
        }

        {
            // sort_by

            let mut v = vec![
                User {
                    name: "foo".to_string(),
                    age: 1,
                },
                User {
                    name: "foo".to_string(),
                    age: 0,
                },
            ];

            v.sort_by(|a, b| {
                let a_key = (&a.name, &a.age);
                let b_key = (&b.name, &b.age);

                a_key.cmp(&b_key)
            });

            assert_eq!(v[0].age, 0);

            // 逆順にソートする場合
            // ・引数の順番を入れ替える
            // ・ソート後にreverseを呼び出す
        }

        {
            // sort_by_key

            let mut v = vec![
                User {
                    name: "foo".to_string(),
                    age: 1,
                },
                User {
                    name: "bar".to_string(),
                    age: 0,
                },
            ];

            v.sort_by_key(|u| u.age);

            assert_eq!(v[0].age, 0);

            // ソートキーの値はソートの際にキャッシュされない
        }

        {
            // reverse
        }

        {
            // binary_search, binary_search_by, binary_search_by_key

            let v = vec![
                User {
                    name: "foo".to_string(),
                    age: 1,
                },
                User {
                    name: "bar".to_string(),
                    age: 0,
                },
            ];

            assert_eq!(v.binary_search_by(|a| a.age.cmp(&0)), Ok(1));
        }

        {
            // contains
        }

        // 浮動小数点数で並べ替えをするには、ord_subsetクレートを使う
    }

    {
        // スライスの比較

        {
            let v1 = vec![0];
            let v2 = vec![0];
            let v3 = vec![1];

            assert_eq!(v1, v2);
            assert_ne!(v1, v3);

            assert!(v1 < v3);
        }

        {
            let v1 = vec![0, 1, 2];
            let v2 = vec![0, 1];
            let v3 = vec![1, 2];

            assert!(&v1.starts_with(&v2));
            assert!(&v1.ends_with(&v3));
        }
    }

    {
        // ランダムな要素

        {
            // choose
        }

        {
            // shuffle
        }
    }
}
