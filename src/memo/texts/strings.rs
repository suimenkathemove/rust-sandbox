fn main() {
    // Stringは&strに参照解決できるので、strに定義されている全てのメソッドはStringでも使うことができる

    {
        // String値の作成

        {
            // &strはCloneを実装することはできない

            // &strはToOwnedトレイトを実装しているため、to_ownedはsliceをコピーした新たなStringを返すことができる
        }
    }

    {
        // 単純な検査

        {
            // len

            // バイト単位の長さを返す
            let s = "あ";
            assert_eq!(s.len(), 3);
        }

        {
            // is_char_boundary

            // バイトオフセットが文字境界になっている場合に真を返す
        }
    }

    {
        // テキストの追加と挿入

        {
            // push

            let mut s = "a".to_string();
            s.push('b');
            assert_eq!(s, "ab".to_string());
        }

        {
            // push_str

            let mut s = "a".to_string();
            s.push_str("bc");
            assert_eq!(s, "abc".to_string());
        }

        {
            // Stringはstd::fmt::Writeを実装している
            // write!マクロやwriteln!マクロで、Stringにフォーマットしたテキストを追加することができる

            use std::fmt::Write;

            let mut s = String::new();
            write!(s, "foo, {}", "bar");
            assert_eq!(s, "foo, bar".to_string());

            // Stringへの書き出しは失敗しないのでunwrapでもいい
        }

        {
            let left = "left".to_string();
            let right = "right".to_string();
            assert_eq!(left + ", " + &right, "left, right");

            // 文字列結合すると、テキストが後方にコピーされるので、性能が大きく低下する

            // これに対し、末尾に追加していく方法は効率が良い
        }

        {
            // 削除

            {
                // clear

                let mut s = "foo".to_string();
                s.clear();
                assert_eq!(s, "");
            }

            {
                // truncate

                let mut s = "あい".to_string();
                s.truncate(3);
                assert_eq!(s, "あ");
            }

            {
                // pop

                let mut s = "あい".to_string();
                assert_eq!(s.pop(), Some('い'));
                assert_eq!(s, "あ");
            }

            {
                // remove

                let mut s = "あいう".to_string();
                assert_eq!(s.remove(3), 'い');
                assert_eq!(s, "あう");
            }

            {
                // drain

                // バイトで指定した範囲のイテレータを返し、イテレータがドロップされるとその範囲の文字を削除する

                let mut s = "あいう".to_string();
                assert_eq!(s.drain(3..6).collect::<String>(), "い");
                assert_eq!(s, "あう");
            }
        }

        {
            // パターン

            // ・char

            // ・String, &str, &&str

            // ・FnMut(char) -> bool

            // ・&[char]
            // スライス中の全ての文字にマッチする
            // リテラルの配列としてリストを書いた場合には、asを用いて型を合わせなければならない

            let s = " a ";
            assert_eq!(s.trim_start_matches(&[' '] as &[char]), "a ");
            assert_eq!(s.trim_start_matches(&[' '][..]), "a ");

            // このようにしないと固定長の配列型だと思ってしまう
        }

        {
            // イテレート

            {
                // chars

                let s = "abcde";
                let v = s.chars().collect::<Vec<_>>();
                assert_eq!(v, vec!['a', 'b', 'c', 'd', 'e']);
            }

            {
                // char_indices

                let s = "あいうえお";
                let v = s.char_indices().collect::<Vec<_>>();
                assert_eq!(
                    v,
                    vec![(0, 'あ'), (3, 'い'), (6, 'う'), (9, 'え'), (12, 'お')]
                );
            }

            {
                // bytes

                assert_eq!(
                    "abcde".bytes().collect::<Vec<_>>(),
                    vec![97, 98, 99, 100, 101]
                );
            }

            {
                // lines

                // \nまたは\r\nで区切る

                // 行末の改行文字は含まれない

                let s = "ab\ncde\r\n";
                let v = s.lines().collect::<Vec<_>>();
                assert_eq!(v, vec!["ab", "cde"]);
            }

            {
                // split

                // マッチした部分が連続している場合や、先頭や末尾にマッチした場合には、その部分に空文字列を生成する

                let s = "abcde";
                let v = s.split('c').collect::<Vec<_>>();
                assert_eq!(v, vec!["ab", "de"]);

                // 連続している場合
                let s = "abbc";
                let v = s.split('b').collect::<Vec<_>>();
                assert_eq!(v, vec!["a", "", "c"]);

                // 先頭の場合
                let s = "abc";
                let v = s.split('a').collect::<Vec<_>>();
                assert_eq!(v, vec!["", "bc"]);

                // 末尾の場合
                let s = "abc";
                let v = s.split('c').collect::<Vec<_>>();
                assert_eq!(v, vec!["ab", ""]);
            }

            {
                // rsplit

                let s = "abcde";
                let v = s.rsplit('c').collect::<Vec<_>>();
                assert_eq!(v, vec!["de", "ab"]);
            }

            {
                // split_terminator, rsplit_terminator

                // パターンがターミネータとして扱われる

                let s = "abc";
                let v = s.split_terminator('c').collect::<Vec<_>>();
                assert_eq!(v, vec!["ab"]);

                // 先頭の場合は空のスライスを作る
                let s = "abc";
                let v = s.split_terminator('a').collect::<Vec<_>>();
                assert_eq!(v, vec!["", "bc"]);
            }

            {
                // splitn, rsplitn

                let s = "a b c d e";
                let v = s.splitn(3, ' ').collect::<Vec<_>>();
                assert_eq!(v, vec!["a", "b", "c d e"]);
            }

            {
                // split_whitespace

                // 連続したホワイトスペースは1つのセパレータとみなされる

                // 末尾のホワイトスペースは無視される

                // ホワイトスペースの定義はchar::is_whitespaceに準じる

                let s = "a b  c\nd\r\ne ";
                let v = s.split_whitespace().collect::<Vec<_>>();
                assert_eq!(v, vec!["a", "b", "c", "d", "e"]);
            }

            {
                // matches, rmatches, match_indices, rmatch_indices

                let s = "aaaabaa";
                let v = s.matches("aa").collect::<Vec<_>>();
                assert_eq!(v, vec!["aa", "aa", "aa"]);
            }
        }

        {
            // トリミング

            // 文字列のトリミングとは、文字列の先頭か末尾からホワイトスペースなどのテキストを削除すること

            {
                // trim, trim_start, trim_end

                let s = " a b\t";

                assert_eq!(s.trim(), "a b");

                assert_eq!(s.trim_start(), "a b\t");

                assert_eq!(s.trim_end(), " a b");
            }

            {
                // trim_matches, trim_matches_start, trim_matches_end

                let s = "aabaa";
                assert_eq!(s.trim_matches('a'), "b");
            }
        }

        {
            // 大文字小文字変換

            let s = "ab";
            assert_eq!(s.to_uppercase(), "AB");
            assert_eq!(s.to_uppercase().to_lowercase(), "ab");
        }

        {
            // 文字列から他の型へパース

            // std::str::FromStrトレイトを実装している型なら、文字列スライスから値をパースする標準の方法がある

            pub trait FromStr: Sized {
                type Err;

                fn from_str(s: &str) -> Result<Self, Self::Err>;
            }

            // 通常の基本型は全てFromStrを実装している？

            assert_eq!(i32::from_str_radix("0", 10), Ok(0));

            // parse
        }

        {
            // 他の型から文字列へパース

            {
                // 人間が理解できる自然な出力形式を持つ型は、std::fmt::Displayトレイトを実装している
                // これにより、format!マクロでフォーマット指定子{}を使えるようになる

                assert_eq!(format!("{}, {}, {}", "", false, 0), ", false, 0");

                // 数値型、char、String、&strは全てDisplayを実装している

                // 型TがDisplayを実装しているなら、Box<T>、Rc<T>、Arc<T>も自動的に実装する

                // VecやHashMapはDisplayを実装していない
                // ユーザが理解できる表示形式が1つに定まらないから
            }

            {
                // ある型がDisplayを実装しているなら、std::str::ToStringトレイトを実装する
            }

            {
                // 標準ライブラリのパブリックな型は全てstd::fmt::Debugトレイトを実装している

                // Debugトレイトは、値を引数として、プログラマにとって補助となる文字列にフォーマットする

                assert_eq!(format!("{:?}", vec![0, 1]), "[0, 1]");

                // コレクション型は全て包括実装を持つ

                // 独自型にもDebugトレイトを実装するべきである
            }
        }

        {
            // UTF-8としてのアクセス

            {
                // as_bytes

                let s = "abcde";
                let b = s.as_bytes();
                assert_eq!(b, &[97, 98, 99, 100, 101]);
            }

            {
                // into_bytes

                let s = "abcde".to_string();
                let v = s.into_bytes();
                assert_eq!(v, vec![97, 98, 99, 100, 101]);
            }
        }

        {
            // UTF-8データからのテキストの作成

            {
                // std::str::from_utf8

                let a = [97];
                let s = std::str::from_utf8(&a).unwrap();
                assert_eq!(s, "a");
            }

            {
                // String::from_utf8

                let v = vec![97];
                let s = String::from_utf8(v).unwrap();
                assert_eq!(s, "a");

                let v = vec![0];
                let r = String::from_utf8(v);
                if let Err(e) = r {
                    assert_eq!(e.into_bytes(), vec![0]);
                }
            }

            {
                // String::from_utf8_lossy

                // UTF-8としておかしい部分は別の文字で置き換える

                let v = [97, 0];
                let s = String::from_utf8_lossy(&v);
                assert_eq!(s, "a\u{0}");
            }

            {
                // std::str::from_utf8_unchecked, String::from_utf8_unchecked

                // 整形式なUTF-8が入っていることが確実な場合に使う

                // バイト列の中身をチェックしない

                let v = [0];
                unsafe {
                    let s = std::str::from_utf8_unchecked(&v);
                    assert_eq!(s, "\u{0}");
                }
            }
        }

        {
            // ヒープ確保の遅延

            fn get_name() -> String {
                std::env::var("USER").unwrap_or_else(|_| "foo".to_string())
            }
            println!("{}", get_name());

            use std::borrow::Cow;

            fn get_name2() -> Cow<'static, str> {
                std::env::var("USER")
                    .map(Cow::Owned)
                    .unwrap_or(Cow::Borrowed("foo"))
            }
            println!("{}", get_name2());

            // 型Tがstd::fmt::Displayトレイトを実装していれば、Cow<'a, T>を出力すると、Tを出力した場合と同じになる

            // Cowは文字列に使われることが多いので、標準ライブラリにCow<'a, str>に対する特別なサポートが入っている
            // Stringと&strからCow<'a, str>への、FromとIntoによる変換が提供されている

            fn _get_name3() -> Cow<'static, str> {
                std::env::var("USER")
                    .map(|v| v.into())
                    .unwrap_or_else(|_| "foo".into())
            }
        }
    }
}
