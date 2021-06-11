fn main() {
    fn triangle(n: i32) -> i32 {
        (1..n + 1).fold(0, |sum, item| sum + item)
    };

    assert_eq!(triangle(3), 6);

    // Rustのイテレータは、コンパイラが素晴らしく高速な機械語コードに変換できるように注意深く設計されている

    {
        // IteratorトレイトとIntoIteratorトレイト

        // イテレータはstd::iter::Iteratorトレイトを実装する値

        trait Iterator {
            type Item;

            fn next(&mut self) -> Option<Self::Item>;
        }

        // ある型に対して何か自然にスキャンする方法があるなら、その型はstd::iter::IntoIteratorトレイトを実装することができる

        trait IntoIterator {
            type Item;

            type IntoIter: Iterator<Item = Self::Item>;

            fn into_iter(self) -> Self::IntoIter;
        }

        // IntoIteratorを実装するすべての型をiterableと呼ぶ

        let v = vec![0, 1, 2];

        for i in &v {
            println!("{}", i);
        }

        let mut iterator = (&v).into_iter();
        while let Some(i) = iterator.next() {
            println!("{}", i);
        }

        // forループに直接イテレータを渡すこともできる
        // すべてのイテレータは自動的にIntoIteratorを実装していて、into_iterメソッドで自分自身を返すようになっている
    }

    {
        // イテレータの作成

        {
            // iterメソッドとiter_mutメソッド

            // 多くのコレクション型が、その型に対する自然なイテレータを返すメソッド
            // forループに任せる以外の方法としては、イテレータを取得する最も一般的な方法

            use std::collections::BTreeSet;

            let mut favorites = BTreeSet::new();
            favorites.insert("foo".to_string());
            favorites.insert("bar".to_string());

            let mut it = favorites.into_iter();

            assert_eq!(it.next(), Some("bar".to_string()));
            assert_eq!(it.next(), Some("foo".to_string()));
            assert_eq!(it.next(), None);

            // コレクションの値に対してinto_iterを呼び出し返されたイテレータがドロップされると、元のコレクションに残っていた要素もドロップされ、抜け殻となったコレクションも捨てられる

            // 可変参照に対するIntoIteratorを実装していない型もある
            // 可変参照のサポートが部分的な場合もある

            // 共有参照や可変参照のIntoIteratorの、iterメソッドとiter_mutメソッドの呼び出しは同じ

            use std::fmt::Debug;

            fn dump<T, U>(t: T)
            where
                T: IntoIterator<Item = U>,
                U: Debug,
            {
                for u in t {
                    println!("{:?}", u);
                }
            };

            // ジェネリック関数をiterやiter_mutで書くことはできない
            // これらのメソッドはトレイトに属しておらず、ほとんどのiterableな型がたまたまこれらの名前のメソッドを持っているだけだから
        }

        {
            // drainメソッド

            // コレクションへの可変参照を借用する

            use std::iter::FromIterator;

            let mut outer = "Earth".to_string();
            let inner = String::from_iter(outer.drain(1..4));

            assert_eq!(outer, "Eh");
            assert_eq!(inner, "art");
        }
    }

    {
        // イテレータアダプタ

        // 1つのイテレータを消費し、何らかの有用な動作を行って、別のイテレータを作る

        {
            // mapとfilter

            let v: Vec<i32> = (0..3).map(|i| i + 1).filter(|i| *i >= 2).collect();
            assert_eq!(v, vec![2, 3]);

            // ほとんどのアダプタがselfを値で受け取るので、SelfがSizedであることが要求される

            // アダプタを呼び出すだけではアイテムを消費せず、新しいイテレータを返すだけ
            // アダプタのチェーンで何かを実際にするには、最後のイテレータに対してnextを呼び出さなければならない
            // 例えば、アダプタの返したイテレータのnextをcollectが呼び始めるまでは何も起きない

            // アダプタはオーバーヘッドのない抽象化である
        }

        {
            // filter_mapとflat_map

            let v = vec!["0", "a"];
            let v2: Vec<i32> = v.iter().filter_map(|i| i.parse().ok()).collect();
            assert_eq!(v2, vec![0]);

            // Noneの値はドロップされ、Some(v)に対してvを生成する

            // ある値を含めるかどうかを決めるのに、実際に処理をしてみないとわからない場合に使う

            let v3: Vec<i32> = v
                .iter()
                .map(|i| i.parse())
                .filter(|r| r.is_ok())
                .map(|r| r.unwrap())
                .collect();
            assert_eq!(v3, vec![0]);

            // flat_mapに与えられるクロージャはiterableな値を返さなければならない
        }

        {
            // scan

            // 状態を表す可変値がクロージャに渡される
            // 繰り返しを打ち切ることができる

            let v: Vec<i32> = (0..10)
                .scan(0, |sum, item| {
                    *sum += item;

                    if *sum > 10 {
                        None
                    } else {
                        Some(item * item)
                    }
                })
                .collect();
            assert_eq!(v, vec![0, 1, 4, 9, 16]);
        }

        {
            // takeとtake_while

            // 途中で停止することができる

            // takeの返すイテレータは、最大n個の要素を生成したらNoneを生成する
            // take_whileの返すイテレータは、個々の要素にpredicateを適用し、predicateがfalseを返す要素が初めて現れた時点でNoneを生成する
        }

        {
            // skipとskip_while

            // takeとtake_whileと対になるもの
        }

        {
            // peekable

            // 次に生成されるアイテムを実際には消費せずに盗み見ることができる

            // PeekableイテレータにはOption<&Item>を返すpeekメソッドがある
            // 下敷きになるイテレータに値があればSome(r)を返す
            // rは次のアイテムへの共有参照

            // peekを呼び出すと、下敷きとなるイテレータから次の値を引き出そうとする
            // もしあれば、次のnextが呼び出されるときまでそれをキャッシュしておく
            // Peekableが持つ、他のIteratorトレイトのメソッドは、すべてこのキャッシュのことを意識するように書かれている

            // イテレータに対して消費するアイテムの数を、実際に見てみるまで決められないような場合に使う

            use std::iter::Peekable;

            fn parse_number<I>(tokens: &mut Peekable<I>) -> u32
            where
                I: Iterator<Item = char>,
            {
                let mut n = 0;

                loop {
                    match tokens.peek() {
                        Some(r) if r.is_digit(10) => {
                            n = n * 10 + r.to_digit(10).unwrap();
                        }
                        _ => return n,
                    }

                    tokens.next();
                }
            }

            let mut chars = "123,456".chars().peekable();
            assert_eq!(parse_number(&mut chars), 123);
            assert_eq!(chars.next(), Some(','));
            assert_eq!(parse_number(&mut chars), 456);
        }

        {
            // fuse

            // 一度Noneを返したIteratorに対して、再度nextメソッドを呼び出した場合の動作をトレイトは規定していない
            // ほとんどのイテレータはNoneを返すようになっているが、すべてがそうなっているとは限らない

            // fuseアダプタは任意のイテレータを、一度Noneを返したら常にNoneを返すイテレータに変換する

            struct Flaky(bool);

            impl Iterator for Flaky {
                type Item = &'static str;

                fn next(&mut self) -> Option<Self::Item> {
                    if self.0 {
                        self.0 = false;
                        Some("")
                    } else {
                        self.0 = true;
                        None
                    }
                }
            }

            let mut flaky = Flaky(true);
            assert_eq!(flaky.next(), Some(""));
            assert_eq!(flaky.next(), None);
            assert_eq!(flaky.next(), Some(""));

            let mut not_flaky = Flaky(true).fuse();
            assert_eq!(not_flaky.next(), Some(""));
            assert_eq!(not_flaky.next(), None);
            assert_eq!(not_flaky.next(), None);
        }

        {
            // 反復可能イテレータとrev

            let v: Vec<i32> = (0..3).collect();

            let mut iter = v.iter();
            assert_eq!(iter.next(), Some(&0));
            assert_eq!(iter.next_back(), Some(&2));

            let mut iter2 = v.iter().rev();
            assert_eq!(iter2.next(), Some(&2));
        }

        {
            // inspect

            let nums: Vec<i32> = (0..3)
                .map(|i| i * 2)
                .inspect(|i| println!("{}", i))
                .collect();
        }

        {
            // chain

            // 同じ型のアイテムを生成するイテレート可能な型と連結する

            let v: Vec<i32> = (0..3).chain(vec![3, 4, 5]).collect();
            assert_eq!(v, [0, 1, 2, 3, 4, 5]);
        }

        {
            // enumerate

            for (i, v) in vec!["a", "b", "c"].iter().enumerate() {
                println!("{} {}", i, v);
            }
        }

        {
            // zip

            let v: Vec<_> = (0..).zip("ABC".chars()).collect();
            assert_eq!(v, vec![(0, 'A'), (1, 'B'), (2, 'C')]);

            // zipの引数はイテレート可能な型でもよい
        }

        {
            // by_ref

            // 一般には、アダプタが下敷きとなるイテレータの所有権を取得するし、イテレータを返却させるメソッドも無いので、アダプタをイテレータから取り外すことはできない

            // by_refはイテレータに対する可変参照を借用する
            // これにより、アダプタをイテレータの参照に対して適用できる
            // アダプタからのアイテムを消費し終えたあとにアダプタをドロップすると、参照の借用も終わるので、元のイテレータがまた利用できる

            let str = "AB\r\n\r\nC";
            let mut lines = str.lines();

            for l in lines.by_ref().take_while(|l| !l.is_empty()) {
                println!("{}", l);
            }

            println!("after");

            for l in lines {
                println!("{}", l);
            }

            // impl<'a, I: Iterator + ?Sized> Iterator for &'a mut I {
            //     type Item = I::Item;

            //     fn next(&mut self) -> Option<I::Item> {
            //         (**self).next()
            //     }

            //     fn size_hint(&self) -> (usize, Option<usize>) {
            //         (**self).size_hint()
            //     }
            // }

            // イテレータ型Iに対して、&mut Iもイテレータになり、そのイテレータのnextメソッドやsize_hintメソッドは、参照先のイテレータにそのままフォワードされる
        }

        {
            // cloned

            // 参照を生成するイテレータに対して、その参照から値をクローンして生成するイテレータを返す
            // 参照されている型はCloneを実装していなければならない

            let v: Vec<i32> = (0..3).collect();

            assert_eq!(v.iter().next(), Some(&0));
            assert_eq!(v.iter().cloned().next(), Some(0));
        }

        {
            // cycle

            // 下敷きとなるイテレータはCloneを実装していなければならない
            // これを用いて、最初の状態を保持しておき、サイクルが回る度に再利用する

            let v: Vec<i32> = (0..3).collect();
            let mut cycleV = v.iter().cycle();
            assert_eq!(cycleV.next(), Some(&0));
            assert_eq!(cycleV.next(), Some(&1));
            assert_eq!(cycleV.next(), Some(&2));
            assert_eq!(cycleV.next(), Some(&0));
        }
    }

    {
        // イテレータの消費

        {
            // count, sum, product

            let r: Vec<i32> = (1..5).collect();

            assert_eq!(r.iter().count(), 4);
            assert_eq!(r.iter().sum::<i32>(), 10);
            assert_eq!(r.iter().product::<i32>(), 24);

            // SumトレイトとProductトレイトを実装すれば他の型でsumやproductを使えるようになる
        }

        {
            // max, min

            let r: Vec<i32> = (1..5).collect();

            assert_eq!(r.iter().min(), Some(&1));
            assert_eq!(r.iter().max(), Some(&4));
        }

        {
            // max_by, min_by

            use std::cmp::Ordering;

            fn cmp(lhs: &&f64, rhs: &&f64) -> Ordering {
                lhs.partial_cmp(rhs).unwrap()
            }

            let nums = [1., 2., 3.];

            assert_eq!(nums.iter().max_by(cmp), Some(&3.));
            assert_eq!(nums.iter().min_by(cmp), Some(&1.));
        }

        {
            // max_by_key, min_by_key

            use std::collections::HashMap;

            let mut map = HashMap::new();

            map.insert("foo", 0);
            map.insert("bar", 1);
            map.insert("baz", 2);

            assert_eq!(
                map.iter().max_by_key(|&(_str, num)| num),
                Some((&"baz", &2))
            );
            assert_eq!(
                map.iter().min_by_key(|&(_str, num)| num),
                Some((&"foo", &0))
            );
        }

        {
            // アイテム列の比較

            let nums1: Vec<i32> = (0..3).collect();
            let nums2: Vec<i32> = (0..3).collect();
            let nums3: Vec<i32> = (3..6).collect();

            assert!(nums1.iter().eq(&nums2));
            assert!(nums1.iter().lt(&nums3));
        }

        {
            // any, all

            let str = "Hoge";

            assert!(str.chars().any(char::is_uppercase));
            assert!(!str.chars().all(char::is_uppercase));

            // 答えを決定するのに必要な数のアイテムしか消費しない
        }

        {
            // position, reposition, ExactSizeIterator

            let str = "hoge";

            assert_eq!(str.chars().position(|c| c == 'o'), Some(1));
            assert_eq!(str.chars().position(|c| c == 'a'), None);

            let bytes = b"aba";
            assert_eq!(bytes.iter().rposition(|&c| c == b'a'), Some(2));

            // rpositionは、反復可能イテレータでないと動作しない
            // さらに、サイズが決定しているイテレータである必要がある
            // サイズが決定しているイテレータは、std::iter::ExactSizeIteratorトレイトを実装している
            // &strに対するcharsイテレータはアイテムの数が事前にはわからないので、文字列に対してrpositionを使うことはできない
        }

        {
            // fold

            let v: Vec<i32> = (1..5).collect();

            assert_eq!(v.iter().fold(0, |n, _| n + 1), 4);
            assert_eq!(v.iter().fold(0, |n, i| n + i), 10);
            assert_eq!(v.iter().fold(1, |n, i| n * i), 24);
            assert_eq!(
                v.iter().fold(i32::min_value(), |n, &i| std::cmp::max(n, i)),
                4
            );
            assert_eq!(
                v.iter().fold(String::new(), |mut s, i| {
                    let is: &str = &i.to_string();
                    s.push_str(is);
                    s
                }),
                "1234"
            );
        }

        {
            // nth

            // アダプタと違ってイテレータの所有権を受け取らないので、何度でも呼び出すことができる

            let mut v = 0..5;

            assert_eq!(v.nth(1), Some(1));
            assert_eq!(v.nth(0), Some(2));
        }

        {
            // last

            // イテレータがNoneを返すまで完全にアイテムを消費し、最後のアイテムを返す

            let mut v = 0..5;
            assert_eq!(v.last(), Some(4));

            // イテレータが反転可能だったとしてもアイテムを先頭からすべて消費する
            // 対象が反転可能ですべて消費する必要がないのなら、以下の方が良い
            let mut v = 0..5;
            assert_eq!(v.rev().next(), Some(4));
        }

        {
            // find

            let v: Vec<i32> = (1..5).collect();

            assert_eq!(v.iter().find(|&i| i == &2), Some(&2));
        }

        {
            // collect, FromIterator

            // collectは、イテレータが適切なアイテム型を返すなら、標準ライブラリにあるすべてのコレクションを作ることができる

            // コレクション型が、イテレータから自分自身を作成する方法を知っている
            // このようなコレクションはstd::iter::FromIteratorを実装しており、collectはこのメソッドを呼び出す便利な方法にすぎない

            trait FromIterator<A>: Sized {
                fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self;
            }

            // あるコレクション型がFromIterator<A>を実装していれば、スタティックメソッドfrom_iterを用いて、型Aのアイテムを生成するイテレート可能な型からそのコレクション型の値を作ることができる
        }

        {
            // Extendトレイト

            // std::iter::Extendトレイトを実装しているコレクション型には、extendメソッドでイテレート可能な型のアイテムを追加することができる

            let mut v: Vec<i32> = (0..2).collect();
            v.extend(&[2]);
            assert_eq!(v, &[0, 1, 2]);

            // 標準のコレクションはすべてExtendを実装している

            trait Extend<A> {
                fn extend<T>(&mut self, iter: T)
                where
                    T: IntoIterator<Item = A>;
            }

            // 標準ライブラリのいくつかの型ではFromIteratorを、新しい空のコレクションを作ってからextendを呼び出して値を追加するように実装している
        }

        {
            // partition

            let v: Vec<i32> = (0..3).collect();
            let (odd, even): (Vec<i32>, Vec<i32>) = v.iter().partition(|&n| n % 2 == 0);

            assert_eq!(odd, vec![0, 2]);
            assert_eq!(even, vec![1]);

            // コレクションの型を指定しなければならない
        }

        {
            // 独自イテレータの実装

            struct I32Range {
                start: i32,
                end: i32,
            }

            impl Iterator for I32Range {
                type Item = i32;

                fn next(&mut self) -> Option<i32> {
                    if self.start >= self.end {
                        return None;
                    }

                    let result = Some(self.start);

                    self.start += 1;

                    result
                }
            }

            for r in (I32Range { start: 0, end: 3 }) {
                println!("{}", r);
            }

            enum BinaryTree<T> {
                Empty,
                NonEmpty(Box<TreeNode<T>>),
            }

            struct TreeNode<T> {
                element: T,
                left: BinaryTree<T>,
                right: BinaryTree<T>,
            }

            struct TreeIter<'a, T: 'a> {
                unvisited: Vec<&'a TreeNode<T>>,
            }

            impl<'a, T: 'a> TreeIter<'a, T> {
                fn push_left_edge(&mut self, mut tree: &'a BinaryTree<T>) {
                    while let BinaryTree::NonEmpty(ref node) = &tree {
                        self.unvisited.push(node);

                        tree = &node.left;
                    }
                }
            }

            impl<T> BinaryTree<T> {
                fn iter(&self) -> TreeIter<T> {
                    let mut iter = TreeIter {
                        unvisited: Vec::new(),
                    };
                    iter.push_left_edge(self);
                    iter
                }
            }

            impl<'a, T: 'a> IntoIterator for &'a BinaryTree<T> {
                type Item = &'a T;

                type IntoIter = TreeIter<'a, T>;

                fn into_iter(self) -> Self::IntoIter {
                    self.iter()
                }
            }

            impl<'a, T> Iterator for TreeIter<'a, T> {
                type Item = &'a T;

                fn next(&mut self) -> Option<&'a T> {
                    let node = match self.unvisited.pop() {
                        None => return None,
                        Some(n) => n,
                    };

                    self.push_left_edge(&node.right);

                    Some(&node.element)
                }
            }

            fn make_node<T>(
                element: T,
                left: BinaryTree<T>,
                right: BinaryTree<T>,
            ) -> BinaryTree<T> {
                BinaryTree::NonEmpty(Box::new(TreeNode {
                    element,
                    left,
                    right,
                }))
            }

            let subtree_l = make_node("l", BinaryTree::Empty, BinaryTree::Empty);
            let subtree_rl = make_node("rl", BinaryTree::Empty, BinaryTree::Empty);
            let subtree_r = make_node("r", subtree_rl, BinaryTree::Empty);
            let tree = make_node("tree", subtree_l, subtree_r);

            let mut v = Vec::new();
            for kind in &tree {
                v.push(*kind);
            }

            assert_eq!(v, ["l", "tree", "rl", "r"]);
        }
    }
}
