fn main() {
    macro_rules! my_assert_eq {
        ($left:expr, $right:expr) => {{
            match (&$left, &$right) {
                (left_val, right_val) => {
                    if *left_val != *right_val {
                        panic!(
                            "assertion failed: `(left == right)`\n left: `{:?}`,\nright: `{:?}`",
                            left_val, right_val
                        )
                    }
                }
            }
        }};
    }

    my_assert_eq!(0, 0);

    // 正規表現が文字に対して操作を行うのに対して、パターンはトークンに対して操作を行う
    // トークンとは、数値や識別子、括弧などのプログラムを構成する要素
    // コメントやホワイトスペースはトークンではない

    macro_rules! my_vec {
        ($elem:expr; $n:expr) => {
            ::std::vec::from_elem($elem, $n)
        };

        ( $( $x:expr ),* ) => {
            <[_]>::into_vec(Box::new([ $( $x ),* ]))
        };
        ( $( $x:expr ),* ) => {
            {
                let mut v = Vec::new();
                $( v.push($x); )*
                v
            }
        };

        ( $( $x:expr ),+ , ) => {
            my_vec![ $( $x ),* ]
        };
    }

    let v1 = my_vec![0; 1];
    assert_eq!(v1, vec![0]);

    let v2 = my_vec![0, 1];
    assert_eq!(v2, vec![0, 1]);

    let v3 = my_vec![0, 1,];
    assert_eq!(v3, vec![0, 1,]);

    {
        // デバッグ

        {
            // cargo build --verboseを使うと、Cargoがrustcを起動する際のオプションが出力される
            // そのオプションに-Z unstable-options --pretty expandedを追加して実行すると、展開されたコードが端末に表示される
        }

        {
            // log_syntax!

            // コンパイル時に引数を端末に表示する

            // #![feature(log_syntax)]を付ける必要がある
        }

        {
            // trace_macros!(true)

            // 全てのマクロ呼び出しを端末に表示する

            // #![feature(trace_macros)]
        }
    }

    {
        // 手続きマクロ
    }

    {
        // ビルドスクリプト

        // コードを生成する別の方法

        // https://doc.rust-lang.org/cargo/reference/build-scripts.html
    }
}
