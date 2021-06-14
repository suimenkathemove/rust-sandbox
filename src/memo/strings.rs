fn main() {
    let s = "ಠ_ಠ";
    assert_eq!(s.len(), 7);
    assert_eq!(s.chars().count(), 3);

    {
        // Unicode

        // Rustの文字と文字列型はUnicodeを対象に設計されている

        {
            // ASCII, Latin-1, Unicode

            fn _latin1_to_char(latin1: u8) -> char {
                latin1 as char
            }

            fn _char_to_latin1(c: char) -> Option<u8> {
                if c as u32 <= 0xff {
                    Some(c as u8)
                } else {
                    None
                }
            }
        }
    }

    {
        // char

        assert!('0'.is_numeric());

        assert!('a'.is_alphabetic());

        assert!('0'.is_alphanumeric());
        assert!('a'.is_alphanumeric());

        assert!(' '.is_whitespace());

        assert!('\n'.is_control());

        {
            // 数字

            // ASCII数字だけ

            {
                // to_digit

                assert_eq!('0'.to_digit(10), Some(0));
                assert_eq!('a'.to_digit(16), Some(10));
                assert_eq!('A'.to_digit(16), Some(10));
            }

            {
                // from_digit

                assert_eq!(std::char::from_digit(0, 10), Some('0'));
                assert_eq!(std::char::from_digit(10, 16), Some('a'));
            }

            {
                // is_digit

                assert_eq!('a'.is_digit(10), 'a'.to_digit(10) != None);
            }
        }

        {
            // 大文字小文字変換

            {
                // is_lowercase, is_uppercase
            }

            {
                // to_lowercase, to_uppercase

                // イテレータを返す

                let mut upper = 'a'.to_uppercase();
                assert_eq!(upper.next(), Some('A'));
                assert_eq!(upper.next(), None);
            }
        }

        {
            // 整数との変換

            assert_eq!('a' as u32, 97);

            // 上位ビットは丸められる

            // as演算子で任意のu8の値をcharにすることができる
            // しかし、よりビット数の大きい整数型は無効なコードポイントを指している可能性があるので、std::char::from_u8を使う

            assert_eq!(std::char::from_u32(97), Some('a'));
        }
    }
}
