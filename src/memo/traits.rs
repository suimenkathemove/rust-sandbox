use std::fmt::Debug;

fn main() {
    // トレイト⋯インターフェイス、もしくは抽象基底クラスのようなもの

    {
        // トレイトの使い方

        // トレイト自体がスコープに入っていないとメソッドは見えない

        {
            // トレイトオブジェクト

            use std::io::Write;

            let mut buf: Vec<u8> = vec![];
            // let writer: Write = buf; // `Write` does not have a constant size
            let writer: &mut Write = &mut buf;
        }

        {
            // トレイトオブジェクトのメモリ配置

            // メモリ上では、トレイトオブジェクトはファットポインタで、値へのポインタと、値の型を表すテーブル（仮想テーブル）へのポインタで構成される
            // 仮想テーブルはコンパイル時に一度だけ作られ、同じ型のすべてのオブジェクトによって共有される
        }

        {
            // トレイトオブジェクトとジェネリック関数のどちらを使うか

            // トレイトオブジェクト
            // 複数の型が入り混じっているコレクションを扱う場合
            // コンパイルされた出力コードの総量を減らすことができる

            // ジェネリック関数
            // スピード
            // すべてのトレイトがトレイトオブジェクトをサポートするとは限らないこと
        }
    }

    {
        // トレイトの定義と実装

        {
            // トレイトと他人の定義した型

            // トレイトか型のどちらかがそのスコープで新たに導入された場合に限り、任意の型に対して任意のトレイトを実装することができる
            // つまり、任意の型に対していつでもトレイトを使ってメソッドを追加することができる

            // 拡張トレイト
            trait IsEmoji {
                fn is_emoji(&self) -> bool;
            }

            impl IsEmoji for char {
                fn is_emoji(&self) -> bool {
                    false
                }
            }

            assert_eq!('$'.is_emoji(), false);

            // 一貫性ルール⋯トレイトを実装する際には、トレイトか型のどちらかが、そのクレートで新たに定義されたものでなければならない
            // これによりトレイトの実装が1つしかないことが保証される
        }

        {
            // トレイトでのSelf

            // Self型を使うトレイトには、トレイトオブジェクトは使えない
        }

        {
            // スタティックメソッド

            // トレイトはスタティックメソッドとコンストラクタを持つことができる

            trait StringSet {
                fn new() -> Self;
                fn from_slice(strings: &[&str]) -> Self;
                fn contains(&self, string: &str) -> bool;
                fn add(&mut self, string: &str);
            }

            fn unknown_words<S: StringSet>() {
                let unknowns = S::new();
            }

            // トレイトオブジェクトはスタティックメソッドをサポートしない
            // トレイトオブジェクトを使いたい場合は、スタティックメソッドに制約where Self: Sizedを追加しなければならない
        }
    }

    {
        // 完全就職メソッド呼び出し

        "hello".to_string();

        // 修飾メソッド呼び出し
        str::to_string("hello");
        ToString::to_string("hello");

        // 完全修飾メソッド呼び出し
        <str as ToString>::to_string("hello");

        // ・同じ名前のメソッドが2つあった場合
        // ・self引数の型が推論できない場合
        // ・関数そのものを関数値として扱う場合
        // ・トレイトメソッドをマクロ内から呼び出す場合
    }

    {
        // 型と型の関係を定義するトレイト

        {
            // 関連型

            fn dump<I>(iter: I)
            where
                I: Iterator,
                I::Item: Debug,
            {
                for (index, value) in iter.enumerate() {
                    println!("{}: {:?}", index, value);
                }
            }

            fn dump2<I>(iter: I)
            where
                I: Iterator<Item = String>,
            {
                for (index, value) in iter.enumerate() {
                    println!("{}: {:?}", index, value);
                }
            }

            // 関連型は、それぞれの実装に関係する型が1つしかない場合には適している
        }
    }
}
