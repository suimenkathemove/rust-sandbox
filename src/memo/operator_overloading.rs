use num::Complex;
use std::ops::{Add, Neg, Not};

fn main() {
    // 演算子オーバーロード

    {
        // 算術演算子

        assert_eq!(1.add(1), 2);

        trait Add2<Rhs = Self> {
            type Output;

            fn add(self, rhs: Rhs) -> Self::Output;
        }

        impl<T> Add2 for Complex<T>
        where
            T: Add<Output = T>,
        {
            type Output = Self;

            fn add(self, rhs: Self) -> Self {
                Complex {
                    re: self.re + rhs.re,
                    im: self.im + rhs.im,
                }
            }
        }

        trait Add3<Rhs = Self> {
            type Output;

            fn add(self, rhs: Rhs) -> Self::Output;
        }

        impl<L, R, O> Add3<Complex<R>> for Complex<L>
        where
            L: Add<R, Output = O>,
        {
            type Output = Complex<O>;

            fn add(self, rhs: Complex<R>) -> Self::Output {
                Complex {
                    re: self.re + rhs.re,
                    im: self.im + rhs.im,
                }
            }
        }

        {
            // 単項演算子

            let x = 1;
            assert_eq!(-x, x.neg());

            let bool = false;
            assert_eq!(!bool, bool.not());

            // 補数演算子
            assert_eq!(x.not(), -2);

            trait Neg {
                type Output;

                fn neg(self) -> Self::Output;
            }

            trait Not {
                type Output;

                fn not(self) -> Self::Output;
            }

            // Complex値の符号反転をジェネリックに実装する
            impl<T, O> Neg for Complex<T>
            where
                T: Neg<Output = O>,
            {
                type Output = Complex<O>;

                fn neg(self) -> Complex<O> {
                    Complex {
                        re: self.re.neg(),
                        im: self.im.neg(),
                    }
                }
            }
        }

        {
            // 二項演算子

            // 数値型は算術演算子とビット演算子を実装している
            // bool型はビット演算子を実装している

            // +演算子を使って、Stringを&strスライスや他のStringとつなげることができる
            // 左辺値が&strであることは許されない
        }

        {
            // 複合代入演算子

            // 複合代入演算子の結果は常にユニット

            // すべての数値型は算術複合代入演算子を実装している
            // 整数型とbool型はビット単位複合代入演算子を実装している

            trait AddAssign<Rhs = Self> {
                fn add_assign(&mut self, rhs: Rhs);
            }

            impl<T> AddAssign for Complex<T>
            where
                T: AddAssign<T>,
            {
                fn add_assign(&mut self, rhs: Complex<T>) {
                    self.re.add_assign(rhs.re);
                    self.im.add_assign(rhs.im);
                }
            }
        }
    }

    {
        // 等価性テスト

        let x = 0;
        let y = 1;

        assert_eq!(x == y, x.eq(&y));
        assert_eq!(x != y, x.ne(&y));

        trait PartialEq<Rhs: ?Sized = Self> {
            fn eq(&self, other: &Rhs) -> bool;
            fn ne(&self, other: &Rhs) -> bool {
                !self.eq(other)
            }
        }

        impl<T: PartialEq> PartialEq for Complex<T> {
            fn eq(&self, other: &Complex<T>) -> bool {
                self.re.eq(&other.re) && self.im.eq(&other.im)
            }
        }

        // 型パラメータは通常sizedであることを要求される

        // 標準ライブラリに含まれている型の中で、PartialEqだがEqではない型はf32とf64だけである

        // ジェネリック型に対して自動生成された実装は、型パラメータに依存する場合がある
    }

    {
        // 順序比較

        enum Ordering {
            Less,
            Equal,
            Greater,
        }

        trait PartialOrd<Rhs = Self>: PartialEq<Rhs>
        where
            Rhs: ?Sized,
        {
            fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;

            fn lt(&self, other: &Rhs) -> bool;
            fn le(&self, other: &Rhs) -> bool;
            fn gt(&self, other: &Rhs) -> bool;
            fn ge(&self, other: &Rhs) -> bool;
        }

        // PartialOrdで実装しなければならないメソッドはpartial_cmpだけである

        // 基本型の中では、浮動小数点数値だけがNoneを返す場合がある

        trait Ord: Eq + PartialOrd<Self> {
            fn cmp(&self, other: &Self) -> Ordering;
        }

        // 標準ライブラリの中では、f32とf64だけがOrdを実装していない
    }

    {
        // IndexとIndexMut

        // 配列は[]演算子を直接サポートしているが、他の型に関しては、a[i]は*a.index(i)の短縮系になる

        trait Index<Idx> {
            type Output: ?Sized;

            fn index(&self, index: Idx) -> &Self::Output;
        }

        trait IndexMut<Idx>: Index<Idx> {
            fn index_mut(&mut self, index: Idx) -> &mut Self::Output;
        }
    }
}
