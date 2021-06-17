fn main() {
    // テンプレート文字列は定数でなければならない

    // format!
    // Stringを作る

    // print!, println!
    // 標準出力ストリームに書き出す

    // write!, writeln!
    // 指定した出力ストリームに書き出す

    // panic!
    // パニックを起こし、標準出力ストリームに書き出す

    {
        // フォーマットパラメータ

        // { which:how }の形を取る
        // 両方とも省略可能

        // whichはインデックスでも名前でもよい

        // howは引数をフォーマットする方法を示す
        // howを書く場合はコロンが必要

        assert_eq!(format!("{1}, {0}", 0, 1), "1, 0");
        assert_eq!(format!("{a}", a = "b"), "b");

        assert_eq!(format!("{:?}", [0]), "[0]");
        assert_eq!(format!("{:?}", "a"), "\"a\"");

        assert_eq!(format!("{{}}"), "{}");
    }

    {
        // テキスト値のフォーマット

        {
            // 最長テキスト長

            assert_eq!(format!("{:.1}", "aa"), "a");
        }

        {
            // 最短フィールド幅

            // 丸めを行った後でこの長さより短ければ、右側（デフォルト）にスペース（デフォルト）でパディングして、この長さにする

            assert_eq!(format!("{:2}", "a"), "a ");
        }

        {
            // アラインメント

            assert_eq!(format!("{:<3}", "a"), "a  ");
            assert_eq!(format!("{:^3}", "a"), " a ");
            assert_eq!(format!("{:>3}", "a"), "  a");
        }

        {
            // パディング

            // パディングを指定する場合は、アラインメントも指定する

            assert_eq!(format!("{:*^3}", "a"), "*a*");
        }
    }

    {
        // 数値のフォーマット

        {
            // 符号

            assert_eq!(format!("{:+}", 1), "+1");
            assert_eq!(format!("{:+}", -1), "-1");
        }

        {
            // 0パディング

            assert_eq!(format!("{:02}", 0), "00");
        }

        {
            // 精度

            // 整数値については無視される

            assert_eq!(format!("{:.1}", 0.12), "0.1");
            assert_eq!(format!("{:.3}", 0.12), "0.120");

            assert_eq!(format!("{:e}", 1000), "1e3");
            assert_eq!(format!("{:E}", 1000), "1E3");
        }

        {
            // 記法

            // #が指定されていると、0b、0o、0x、0Xが前に付く
        }
    }

    {
        // 他の型のフォーマット

        // ・エラー型
        // 全てのエラー型が実装している、std::error::Errorトレイトが、std:fmt::Displayを拡張している

        // ・IPアドレス型

        // ・真偽値

        // 文字列と同じフォーマットパラメータを使うことができる
    }

    {
        // デバッグのためのフォーマット

        // 標準ライブラリのパブリック型の全てに対して、{:?}を使うことができる

        let v = vec![0];
        assert_eq!(format!("{:?}", v), "[0]");
        assert_eq!(format!("{:#?}", v), "[\n    0,\n]");

        #[derive(Debug)]
        struct User {
            name: String,
        }
        let user = User {
            name: "foo".to_string(),
        };
        assert_eq!(format!("{:?}", user), "User { name: \"foo\" }");
    }

    {
        // デバッグのためのポインタのフォーマット

        // ポインタをフォーマットマクロに与えると、マクロがポインタの参照をたどって、参照先をフォーマットする

        use std::rc::Rc;

        let original = Rc::new("foo".to_string());
        let _cloned = original.clone();
        let _impostor = Rc::new("foo".to_string());

        // println!("text:     {}, {}, {}", original, _cloned, _impostor);
        // println!("pointers: {:p}, {:p}, {:p}", original, _cloned, _impostor);
    }

    {
        // 動的なフィールド幅

        // 1$と書くと、3つ目の引数を使う
        assert_eq!(format!("{:1$}", "a", 2), "a ");
        assert_eq!(format!("{:.1$}", "aa", 1), "a");

        // テキスト長の制限や浮動小数点精度の部分に*と書くと、2つ目の引数を使う
        // フィールド幅には使えない
        assert_eq!(format!("{:.*}", 1, "aa"), "a");

        // 指定された引数はusize型でなければならない
    }

    {
        // 独自型のフォーマット出力

        // フォーマット出力マクロは、std::fmtモジュールに定義されている一連のトレイトを用いて、テキストへの変換を行う
        // 独自型にフォーマットマクロを使うには、これらのトレイトのうちのいくつかを実装すればよい

        // フォーマット用のトレイトは、名前が違うだけで、全て同じ構造をしている

        // 別の表示形式を選択するためには、#を用いることが多い

        trait Display {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result;
        }

        struct User {
            name: String,
        }

        impl std::fmt::Display for User {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                if f.alternate() {
                    write!(f, "Hello, {}!", self.name)
                } else {
                    write!(f, "Hi, {}!", self.name)
                }
            }
        }

        let user = User {
            name: "foo".to_string(),
        };

        assert_eq!(format!("{}", user), "Hi, foo!");
        assert_eq!(format!("{:#}", user), "Hello, foo!");
    }

    {
        // フォーマット言語の独自コードでの利用

        // format_args!マクロとstd::fmt::Arguments型を用いて、フォーマットテンプレートと引数を受け付ける独自の関数やマクロを記述することができる
    }
}
