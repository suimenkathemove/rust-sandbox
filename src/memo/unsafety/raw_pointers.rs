fn main() {
    // Tへのrawポインタ
    // *mut T⋯書き込み可能
    // *const T⋯読み出し専用

    {
        // rawポインタは参照から変換して作ることが可能

        // *演算子で参照解決することができる

        let mut x = 1;
        let ptr_x = &mut x as *mut i32;

        let y = Box::new(2);
        let ptr_y = &*y as *const i32;

        unsafe {
            *ptr_x += *ptr_y;
        }

        assert_eq!(x, 3);
    }

    {
        // rawポインタはnullの可能性がある

        fn option_to_raw<T>(opt: Option<&T>) -> *const T {
            match opt {
                None => std::ptr::null(),
                Some(r) => r as *const T,
            }
        }

        assert_eq!(option_to_raw::<i32>(None), std::ptr::null());

        // rawポインタの参照解決のみがunsafe
    }

    {
        // 演算子はrawポインタをアドレスとして比較する

        let foo = Box::new(0);
        let foo_ptr = &*foo as *const i32;
        let foo_ptr2 = &*foo as *const i32;

        let bar = Box::new(0);
        let bar_ptr = &*bar as *const i32;

        assert_ne!(foo_ptr, bar_ptr);
        assert_eq!(foo_ptr, foo_ptr2);
    }

    {
        let foo = Box::new(0);
        let _ptr_foo = &*foo as *const i32;

        // フォーマットトレイトは、基本的には利用できない
        // println! {"{}", ptr_foo}; // error

        // 16進数のアドレスとして表示する
        // println! {"{:?}", _ptr_foo};
        // println! {"{:p}", _ptr_foo};
    }

    {
        // +演算子はrawポインタに対して使用できないが、offsetやwrapping_offsetを使ってポインタ演算を行うことはできる

        // -演算子のような、2つのポインタ間の距離を計算する標準の演算子は無いが、自分で書くことはできる

        fn distance<T>(left: *const T, right: *const T) -> isize {
            (left as isize - right as isize) / std::mem::size_of::<T>() as isize
        }

        let v = (0..3).collect::<Vec<_>>();
        let first = v.first().unwrap();
        let last = v.last().unwrap();

        assert_eq!(distance(last, first), 2);
        assert_eq!(distance(first, last), -2);

        // 暗黙に参照をrawポインタに型変換するが、逆は行わない
    }

    {
        // rawポインタを安全に参照解決するには

        // ・参照対象の型として適切にアラインされていないポインタを参照解決すると未定義動作となる

        // ・rawポインタを参照解決して得た値から、参照を借用していいのは、参照の安全性のためのルールに従っている場合のみ

        // ・rawポインタに対してoffsetやwrapping_offsetを使う場合には、その結果が、元のポインタが参照している変数もしくはヒープ上のメモリブロックの中のバイト列か、そのような領域の次のバイトを指すようにしなければならない
        // ポインタを変数に変換して、整数上で演算を行い、またポインタに戻すことでポインタ演算を行う場合には、その結果のポインタは、offsetのルールが許すようなポインタでなければならない

        // ・rawポインタの参照先に代入を行う場合には、参照先がその一部となる型の不変条件に違反してはならない
        // 例えば、String中のバイト値を指している*mut u8に対しては、元のStringが整形式なUTF-8のままでいられるような値しか書き込んではならない
    }

    {
        // 例：RefWithFlag

        // ガベージコレクションや仮想計算機で、例えばあるオブジェクトを表している型の値があまりにも多く、それぞれに１ワードずつ追加するとメモリ消費量が劇的に増えてしまうような場合によく用いられる

        mod ref_with_flag {
            use std::marker::PhantomData;
            use std::mem::align_of;

            pub struct RefWithFlag<'a, T: 'a> {
                ptr_and_bit: usize,
                behaves_like: PhantomData<&'a T>,
            }

            impl<'a, T: 'a> RefWithFlag<'a, T> {
                pub fn new(ptr: &'a T, flag: bool) -> RefWithFlag<T> {
                    assert!(align_of::<T>() % 2 == 0);

                    RefWithFlag {
                        ptr_and_bit: ptr as *const T as usize | flag as usize,
                        behaves_like: PhantomData,
                    }
                }

                pub fn get_ref(&self) -> &'a T {
                    unsafe {
                        let ptr = (self.ptr_and_bit & !1) as *const T;
                        &*ptr
                    }
                }

                pub fn get_flag(&self) -> bool {
                    self.ptr_and_bit & 1 != 0
                }
            }
        }

        // 偶数アドレスの最下位ビットは常にゼロなので、そこに何か別のものを格納することができる

        // rawポインタの参照先から参照を借用すると、参照の生存期間が無制限になってしまう
        // Rustは周辺のコードが許す生存期間ならどんなものでも受け入れてしまう

        // PhantomDataはゼロサイズの型で、behaves_likeフィールドは構造体上で全くメモリを消費しない
        // しかし、RefWithFlagを使うコードで、生存期間をどう扱うかをRustに教えるために必要である

        use ref_with_flag::RefWithFlag;

        let v = vec![10, 20, 30];
        let flagged = RefWithFlag::new(&v, true);
        assert_eq!(flagged.get_ref()[1], 20);
        assert_eq!(flagged.get_flag(), true);
    }

    {
        // 型のサイズとアラインメント

        // Sized型の値は、メモリ上に一定のバイト数を専有し、何らかのアラインメント値の倍数のアドレスに配置される

        // i32のバイト数
        assert_eq!(std::mem::size_of::<i32>(), 4);

        // アラインメント値
        assert_eq!(std::mem::align_of::<i32>(), 4);

        // アラインメントは常に2の冪乗
        // 型のサイズは、常にアラインメントの倍数に繰り上げられる
        // これにより、配列に並べたときに、要素型のサイズが、ある要素と次の要素の間の空間を常に反映するようになる

        // unsized型については、サイズとアラインメントは個々の値に依存する
    }

    {
        // ポインタ演算

        // 配列に対する繰り返し処理を行う際の境界チェックにrawポインタを使うことができる
    }

    {
        // メモリに移動、メモリから移動

        // メモリを管理する型を実装する場合には、メモリのどの部分が生きている値を指していて、どの部分が未初期化状態なのかを管理する必要がある

        let s1 = "".to_string();
        let _s2;

        // 代入後もs1には文字列のポインタと容量と長さが格納されたままである
        _s2 = s1;

        // 初期化済みの値の定義は、生きているものとして扱われていること

        {
            // 独自にメモリを管理する型を実装するための基本的な操作

            // std::ptr::read(src)
            // std::ptr::write(dst, src)

            // std::ptr::read_unaligned(src)
            // std::ptr::write_unaligned(dst, src)

            // std::ptr::read_volatile(src)
            // std::ptr::write_volatile(dst, src)
        }

        {
            // 値の配列を、あるメモリブロックから別のメモリブロックに移動する関数

            // std::ptr::copy(src, dst, count)

            // std::ptr::copy_nonoverlapping(src, dst, count)
        }
    }
}
