use std::{borrow::Borrow, collections::HashSet, fmt::Display};

fn main() {
    {
        // Drop

        // 値をドロップするには、その値が所有する他の値や、ヒープ上のストレージ、システムリソースなどをすべて解放しなければならない
        // ドロップは、変数がスコープから出たとき、式の値が;演算子で捨てられたとき、ベクタの後ろの要素を捨てて短くしたときなど、さまざまな状況で発生する

        struct Appellation {
            name: String,
            nicknames: Vec<String>,
        }

        // std::ops::Dropを実装している値がドロップされる場合は、その前にdropメソッドが呼び出される

        impl Drop for Appellation {
            fn drop(&mut self) {
                print!("{}", self.name);

                if !self.nicknames.is_empty() {
                    print!("{}", self.nicknames.join(", "));
                }

                println!("");
            }
        }

        let mut a = Appellation {
            name: "Zeus".to_string(),
            nicknames: vec![
                "cloud collector".to_string(),
                "king of the gods".to_string(),
            ],
        };

        a = Appellation {
            name: "Hera".to_string(),
            nicknames: vec![],
        };

        println!("at end of block");

        // 変数の値が別の場所に移動されていて、変数のスコープが終わる際に未初期化状態になっていれば、その変数はドロップされない

        // 値は移動されるかもしれないが、ドロップされるのは一度だけ

        // Rustが知らない資源を管理する型を定義しているのでもない限り、std::ops::Dropを実装する必要はない

        // DropトレイトとCopyトレイトは同時に実装できない
    }

    {
        // Sized

        // sized型は、その型のメモリ上での値のサイズが常に同じになる

        // 文字列スライスstrや、配列スライス[T]はunsized

        // トレイトオブジェクトの参照先もunsized
        // Writeを実装するかもしれない型の集合は閉じていないので、Writeはunsizedになる

        // unsizedの値を変数に格納したり、引数として渡すことはできない
        // &strやBox<Write>のようにポインタを介して扱うことしかできない

        // 構造体の最後のフィールドはunsizedでもよい
        // そのような構造体はそれ自身unsizedになる

        struct RcBox<T: ?Sized> {
            ref_count: usize,
            value: T,
        }

        let boxed_lunch: RcBox<String> = RcBox {
            ref_count: 1,
            value: "lunch".to_string(),
        };

        let boxed_displayable: &RcBox<Display> = &boxed_lunch;

        fn display(boxed: &RcBox<Display>) {
            println!("{}", &boxed.value);
        }

        display(&boxed_lunch);
    }

    {
        // Clone

        trait Clone: Sized {
            fn clone(&self) -> Self;

            fn clone_from(&mut self, source: &Self) {
                *self = source.clone();
            }
        }

        // ある値のクローンを作ることは、それが保有しているものもすべてコピーすることになる
    }

    {
        // Copy

        // ある型がCopy型となるのはstd::marker::Copyマーカトレイトを実装している場合である

        trait Copy: Clone {}

        struct MyType {}

        // impl Copy for MyType {}

        // Copyマーカトレイトには言語上で特殊な意味を持つので、Copyを実装できる値は、浅いバイト単位のコピーだけでコピー可能な型のみに制限されている

        // Dropトレイトを実装している型はCopyにすることはできない
        // ある型が特別な後始末用のコードを必要とするならば、コピーする際にも何らかの特別な方法が必要なはずだから
    }

    {
        // DerefとDerefMut

        trait Deref2 {
            type Target: ?Sized;

            fn deref(&self) -> &Self::Target;
        }

        trait DerefMut2: Deref2 {
            fn deref_mut(&mut self) -> &mut Self::Target;
        }

        // 参照解決型変換⋯derefを呼び出すことで型の不整合が防げるのなら、自動的にderefを呼び出す

        struct Selector<T> {
            elements: Vec<T>,
            current: usize,
        }

        use std::ops::{Deref, DerefMut};

        impl<T> Deref for Selector<T> {
            type Target = T;

            fn deref(&self) -> &T {
                &self.elements[self.current]
            }
        }

        impl<T> DerefMut for Selector<T> {
            fn deref_mut(&mut self) -> &mut T {
                &mut self.elements[self.current]
            }
        }

        let mut s = Selector {
            elements: vec!['x', 'y', 'z'],
            current: 2,
        };

        assert_eq!(*s, 'z');

        assert!(s.is_alphabetic());

        *s = 'w';
        assert_eq!(s.elements, ['x', 'y', 'w']);

        // DerefトレイトとDerefMutトレイトは、Box、Rc、Arcなどのスマートポインタ型を実装するために設計されている
        // また、参照で使う場合が多い型の「所有バージョン」を実装するのにも適している
        // Target型のメソッドを自動的に使うためだけにDerefやDerefMutを実装してはいけない

        // 型の不整合を解決するためにこの機能は使われるが、型変換の制約を満たすためには用いられない

        fn show_it_generic<T: Display>(thing: T) {
            println!("{}", thing);
        }
        show_it_generic(&s as &char);
    }

    {
        // Default

        trait Default {
            fn default() -> Self;
        }

        impl Default for String {
            fn default() -> String {
                String::new()
            }
        }

        let squares = [1, 4, 9, 16, 25];
        let (powers_of_two, impure): (HashSet<i32>, HashSet<i32>) =
            squares.iter().partition(|&n| n & (n - 1) == 0);
        assert_eq!(powers_of_two.len(), 3);
        assert_eq!(impure.len(), 2);

        let (upper, lower): (String, String) = "Great Teacher Onizuka"
            .chars()
            .partition(|&c| c.is_uppercase());
        assert_eq!(upper, "GTO");

        // ある型TがDefaultを実装していれば、標準ライブラリが自動的にRc<T>、Arc<T>、Box<T>、Cell<T>、RefCell<T>、Cow<T>、Mutex<T>、RwLock<T>についてもDefaultを実装してくれる

        // タプルのすべての要素がDefaultを実装していれば、個々の要素のデフォルト値を保持したタプルがデフォルト値となる

        // 構造体が暗黙にDefaultを実装されることはないが、すべてのフィールドがDefaultを実装していれば#[derive(Default)]を使って自動的にDefaultを実装させることができる
    }

    {
        // AsRefとAsMut

        // ある型がAsRef<T>を実装している場合、その型から&Tを効率的に借用できる

        trait AsRef<T: ?Sized> {
            fn as_ref(&self) -> &T;
        }

        trait AsMut<T: ?Sized> {
            fn as_ref(&mut self) -> &mut T;
        }

        // Vec<T>はAsRef<[T]>を、StringはAsRef<str>を実装している
        // Stringの内容をバイトの配列として借用できるように、StringはAsRef<[u8]>も実装している

        // AsMut<T>を実装して良いのは、Tを変更しても型の不変な性質を侵害しない場合だけ
    }

    {
        // BorrowとBorrowMut

        // std::borrow::BorrowトレイトはAsRefトレイトに似ている
        // ある型がBorrow<T>を実装しているなら、borrowメソッドで&Tを効率的に借用することができる
        // &Tのハッシュ値や比較が元の値と同じように行える場合にだけ、Borrow<T>を実装するべきという制限（Rustはこれを強制せず、トレイトの意図としてドキュメントに書かれているだけ）

        // Borrowの定義はAsRefと全く同じである

        trait Borrow<Borrowed: ?Sized> {
            fn borrow(&self) -> &Borrowed;
        }

        // use std::collections::HashMap;
        // use std::hash::Hash;

        // impl HashMap<K, V>
        // where
        //     K: Eq + Hash,
        // {
        //     fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
        //     where
        //         K: Borrow<Q>,
        //         Q: Eq + Hash,
        //     {
        //     }
        // }

        // 標準ライブラリには包括的な実装があり、すべての型はそれ自身からBorrowできることになっている
        // T: Borrow<T>

        // 簡便化のためにすべての&mut T型がBorrow<T>を実装するようになっている

        trait BorrowMut<Borrowed: ?Sized>: Borrow<Borrowed> {
            fn borrow_mut(&mut self) -> &mut Borrowed;
        }
    }

    {
        // FromとInto

        // ある型の値を消費して別の型を返す変換を表す
        // 引数の所有権を受け取り、変換を行い、呼び出し元に変換結果の所有権を返す

        trait Into<T>: Sized {
            fn into(self) -> T;
        }

        trait From<T>: Sized {
            fn from(_: T) -> Self;
        }

        // 標準ライブラリは、それ自身の型への自明な変換を自動的に実装してくれる
        // つまり、すべての型TがFrom<T>とInto<T>を実装している

        // 適切なFromの実装があれば、標準ライブラリが自動的に対応するIntoトレイトを実装してくれる

        let text = "hoge".to_string();
        let bytes: Vec<u8> = text.into();

        // 失敗しないことが保証されている変換にしか使えない
    }

    {
        // ToOwned

        // 参照から所有された値へ変換するための少し緩和された方法を提供する

        trait ToOwned {
            type Owned: Borrow<Self>;

            fn to_owned(&self) -> Self::Owned;
        }
    }

    {
        // BorrowとToOwnedの動作例：つつましいCow

        // 関数の仮引数を借用するべきか所有するべきかが、実行時まで決められないような場合に使う

        enum Cow<'a, B: ?Sized + 'a>
        where
            B: ToOwned,
        {
            Borrowed(&'a B),
            Owned(<B as ToOwned>::Owned),
        }

        // to_mutメソッドを呼び出すことで、可変参照を取得することもできる
    }
}
