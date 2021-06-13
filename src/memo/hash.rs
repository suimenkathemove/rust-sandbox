use std::{
    collections::HashSet,
    hash::{BuildHasher, Hash, Hasher},
};

fn main() {
    // Eqを実装する組み込み型のほとんどはHashも実装している

    // 標準ライブラリの原則として、ある値のハッシュ値は、どこに格納されていても、どのように指されていても、同じでなければならない

    // 構造体や列挙型にHashを自動実装させることができる
    // 型の全てがハッシュ可能である場合にしかできない

    // ある型に対して、PartialEqを独自に実装したなら、Hashも独自に実装する必要がある

    // ハッシュテーブルでは常に、a == bならばhash(a) == hash(b)でなければならない

    #[derive(Clone, PartialEq, Eq, Hash)]
    enum Id {}

    struct Foo {
        id: Id,
    }

    impl PartialEq for Foo {
        fn eq(&self, other: &Foo) -> bool {
            self.id == other.id
        }
    }

    impl Eq for Foo {}

    impl Hash for Foo {
        fn hash<H: Hasher>(&self, hasher: &mut H) {
            self.id.hash(hasher);
        }
    }

    let mut _hs = HashSet::<Foo>::new();

    {
        // ハッシュアルゴリズムのカスタマイズ

        // HashとHasherはバディトレイト

        // BuildHasherはハッシュアルゴリズムの初期状態を表現する型のためのトレイト
        // 個々のHasherはイテレータと同様に一度しか使われないが、BuildHasherは再利用される

        // HashMapはBuildHasherを持っており、ハッシュ値を計算する必要が生じるとこれを用いる

        // BuildHasherには、キーと、初期状態と、ハッシュアルゴリズムを実行するために必要となる様々なパラメータが含まれる

        fn _compute_hash<B, T>(builder: &B, value: &T) -> u64
        where
            B: BuildHasher,
            T: Hash,
        {
            let mut hasher = builder.build_hasher();
            value.hash(&mut hasher);
            hasher.finish()
        }

        // HashMapはこのメソッドを、ハッシュ値を計算する必要ができる度に呼び出す

        // HashDosに対する安全性を犠牲にして、より高速なハッシュ関数を実装することもできる
    }
}
