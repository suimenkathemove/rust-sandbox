fn main() {
    let mut a: usize = 0;
    let ptr = &mut a as *mut usize;
    unsafe {
        // rawポインタ型は元の参照先の範囲を超えて参照解決してはいけないという契約があるが、その契約を破っている
        *ptr.offset(3) = 0x7ffff72f484c;
    }

    // unsafeな機能は、何らかの契約を課す
    // 契約とは、Rustが自動的に強制することができない、未定義動作を避けるために従わなければならないルール

    {
        // unsafeブロック

        // ・unsafe関数を呼び出すことができる
        // 個々のunsafe関数は目的に応じて契約を指定しなければならない

        // ・rawポインタを参照解決できる

        // ・可変static変数にアクセスできる

        // ・外部関数インターフェイスを通じて関数や変数にアクセスできる

        {
            // 効率的なASCII文字列型

            mod my_ascii {
                #[derive(Debug, Eq, PartialEq)]
                pub struct Ascii(Vec<u8>);

                impl Ascii {
                    pub fn from_bytes(bytes: Vec<u8>) -> Result<Ascii, NotAsciiError> {
                        if bytes.iter().any(|&byte| !byte.is_ascii()) {
                            return Err(NotAsciiError(bytes));
                        }

                        Ok(Ascii(bytes))
                    }

                    pub unsafe fn from_bytes_unchecked(bytes: Vec<u8>) -> Ascii {
                        Ascii(bytes)
                    }
                }

                // This should implement `std::error:Error`.
                #[derive(Debug, Eq, PartialEq)]
                pub struct NotAsciiError(pub Vec<u8>);

                impl From<Ascii> for String {
                    fn from(ascii: Ascii) -> Self {
                        unsafe { String::from_utf8_unchecked(ascii.0) }
                    }
                }
            }

            use my_ascii::Ascii;

            let bytes: Vec<u8> = b"Hello, world!".to_vec();
            let ascii: Ascii = Ascii::from_bytes(bytes).unwrap();
            let string = String::from(ascii);
            assert_eq!(string, "Hello, world!");

            // ニュータイプ⋯内容に対するルールを強制するモジュール内部に、アイテムを隠すラッパ
        }
    }

    {
        // unsafe関数

        // unsafeブロックに入る前に発生したバグによって、契約が破られる場合がある

        // 契約を破ったことの結果は、unsafeブロックから出た後で現れることがある
    }

    {
        // unsafeブロックか、unsafe関数か

        // コンパイルは通るが、未定義動作が起きるような使い方ができてしまう場合は、unsafe関数にする
        // 関数を正しく使うために従うべきルールがある場合は、それが契約になり、契約がある場合はunsafe関数にしなければならない

        // 関数がunsafeな機能を使っているかは関係なく、契約があるかが重要である
        // unsafeな機能を使っていないのにunsafeな関数もある、逆も然り

        // 関数ボディ全体が1つのunsafeブロックに入るような場合でも、unsafe関数ではなくunsafeブロックを使うべきである
    }

    {
        // 未定義動作

        let i = 10;
        very_trustworthy(&i);
        println!("{}", i * 100);

        fn very_trustworthy(shared: &i32) {
            unsafe {
                let mutable = shared as *const i32 as *mut i32;
                *mutable = 20;
            }
        }

        // Rustにおける正しく振る舞うプログラムのルール

        // ・初期化されていないメモリを読んではならない

        // ・無効な基本型を作ってはならない
        // 　・nullを指す参照やBox
        // 　・0でも1でもないbool値
        // 　・無効な判別値を持つenum値
        // 　・有効でなく、サロゲートでもないUnicodeコードポイントを指すchar値
        // 　・整形式でないUTF-8を持つstr値

        // ・nullポインタ、不適切にアラインされたポインタを参照解決してはならない

        // ・ポインタが関連付けられた、メモリ確保された領域の、外を、そのポインタでアクセスしてはならない

        // ・データ競合を起こしてはならない
        // データ競合⋯2つのスレッドが同じメモリ領域を同期せずにアクセスし、そのうち少なくとも一方が書き込みだった場合に発生する

        // ・外部言語インターフェイスを通じて他の言語から行われた呼び出しのスタックを巻き戻してはならない

        // ・標準ライブラリ関数の課す契約に従わなければならない
    }
}
