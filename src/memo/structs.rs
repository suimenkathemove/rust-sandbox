fn main() {
    // Rustには、名前付きフィールド型、タプル型、ユニット型の3種類の構造体がある

    {
        // 名前付きフィールド型構造体

        struct GrayscaleMap {
            pixels: Vec<u8>,
            size: (usize, usize),
        }

        let width = 1024;
        let height = 576;
        let image = GrayscaleMap {
            pixels: vec![0; width * height],
            size: (width, height),
        };

        // 外部モジュールからは、構造体とパブリックメソッドは使えるが、プライベートなフィールドを名前でアクセスすることはできない
        // 構造体の値を作るには、構造体の全てのフィールドが見えていなければならない
        // StringやVecを構造体式では作れないのは、これらの標準の型は構造体だが、全てのフィールドがプライベートになっているから
        // これらの型を作るには、Vec::new()のようなパブリックメソッドを呼ぶ必要がある
    }

    {
        struct Broom {
            name: String,
            height: u32,
            health: u32,
            position: (f32, f32, f32),
            intent: BroomIntent,
        }

        #[derive(Copy, Clone)]
        enum BroomIntent {
            FetchWater,
            DumpWater,
        }

        fn chop(b: Broom) -> (Broom, Broom) {
            let mut broom1 = Broom {
                height: b.height / 2,
                ..b
            };
            let mut broom2 = Broom {
                name: broom1.name.clone(),
                ..broom1
            };

            broom1.name.push_str(" I");
            broom2.name.push_str(" II");

            (broom1, broom2)
        }

        let hokey = Broom {
            name: "Hokey".to_string(),
            height: 60,
            health: 100,
            position: (100.0, 200.0, 0.0),
            intent: BroomIntent::FetchWater,
        };

        let (hokey1, hokey2) = chop(hokey);

        assert_eq!(hokey1.name, "Hokey I");
        assert_eq!(hokey2.name, "Hokey II");
    }

    {
        // タプル型構造体

        struct Bounds(usize, usize);

        let image_bounds = Bounds(1024, 768);

        // 型を定義すると関数が自動的に定義される
        // fn Bounds(el0: usize, el1: usize) -> Bounds {}

        // 名前付きフィールド型構造体とタプル型構造体のどちらを使うかは、読みやすさ、曖昧さ、簡潔さの問題である
        // パターンマッチングで構成要素にアクセスすることが多いなら、タプル型構造体でも良いかもしれない

        // タプル型構造体は、構成要素が1つだけの構造体である「新しい型」を作るのに便利
        struct Ascii(Vec<u8>);
    }

    {
        // ユニット型構造体

        // 全く要素を宣言しない構造体

        struct Onesuch;

        let o = Onesuch;

        // 範囲演算子..の両端の値を省略した式..は、ユニット型構造体RangeFull値の短縮系
    }

    {
        // 構造体のレイアウト

        // メモリ上では、名前付きフィールド型構造体と、タプル型構造体は同じ
        // 構造体のフィールドや要素の配置順について何も規定されていない
        // フィールドの値は構造体のメモリブロックに直接置かれる
    }

    {
        // implによるメソッド定義

        struct Queue {
            older: Vec<char>,
            younger: Vec<char>,
        }

        impl Queue {
            fn push(&mut self, c: char) {
                self.younger.push(c);
            }

            fn pop(&mut self) -> Option<char> {
                if self.older.is_empty() {
                    if self.younger.is_empty() {
                        return None;
                    }

                    use std::mem::swap;
                    swap(&mut self.older, &mut self.younger);

                    self.older.reverse();
                }

                self.older.pop()
            }
        }

        // メソッドを呼び出すときに、自分で可変参照を借用する必要はない

        let mut q = Queue {
            older: Vec::new(),
            younger: Vec::new(),
        };

        q.push('0');
        assert_eq!(q.pop(), Some('0'));
    }

    {
        // ジェネリック構造体

        struct Queue<T> {
            older: Vec<T>,
            younger: Vec<T>,
        }

        impl<T> Queue<T> {
            fn new() -> Self {
                Queue {
                    older: Vec::new(),
                    younger: Vec::new(),
                }
            }

            fn push(&mut self, c: T) {
                self.younger.push(c);
            }

            fn pop(&mut self) -> Option<T> {
                if self.older.is_empty() {
                    if self.younger.is_empty() {
                        return None;
                    }

                    use std::mem::swap;
                    swap(&mut self.older, &mut self.younger);

                    self.older.reverse();
                }

                self.older.pop()
            }
        }

        // 関数のシグネチャと型宣言では常に型パラメータを与えなければならず、コンパイラはこれらの部分を推論しない
        // これらの部分で明示的に指定された型を使って、関数ボディ部の型を推論する
    }

    {
        // ライフタイムパラメータを持つ構造体

        // 構造体型が参照を含むのであれば、参照のライフタイムを指定する必要がある

        // 任意のライフタイム'eltに対して、ライフタイムが'eltの参照を保持するExtrema<'elt>
        struct Extrema<'elt> {
            greatest: &'elt i32,
            least: &'elt i32,
        }

        fn find_extrema<'s>(slice: &'s [i32]) -> Extrema<'s> {
            let mut greatest = &slice[0];
            let mut least = &slice[0];

            for i in 1..slice.len() {
                if slice[i] < *least {
                    least = &slice[i];
                }
                if slice[i] > *greatest {
                    greatest = &slice[i];
                }
            }

            Extrema { greatest, least }
        }
    }

    {
        // 一般的なトレイトの自動実装

        #[derive(Copy, Clone, Debug, PartialEq)]
        struct Point {
            x: f64,
            y: f64,
        }
        // これらのトレイトは、すべてのフィールドがこれらのトレイトを実装しているなら、自動的に実装できる

        let p1 = Point { x: 0., y: 0. };
        let p2 = Point { x: 0., y: 0. };
        assert_eq!(p1, p2);

        // トレイトを実装すると自動的にパブリックになってしまう
    }

    {
        // 内部可変性

        // Cell<T>構造体は型Tのプライベートな値を1つだけ持つ
        // Cellそのものに対するmutな参照を持っていなくても、そのフィールドを見たりセットすることができる
        // 共有されている変数に対してmutメソッドを呼ばせてくれない
        // .get()はセルに収められた値のコピーを返すので、Copyトレイトを実装したTにしか使えない

        // RefCellはT値への参照の借用をサポートしている
        use std::cell::RefCell;

        let ref_cell: RefCell<String> = RefCell::new("hello".to_string());

        let r = ref_cell.borrow();
        assert_eq!(r.len(), 5);

        let mut w = ref_cell.borrow_mut(); // panic: already borrowed
        w.push_str(" world");

        // 通常の参照はコンパイル時にチェックするが、RefCellの場合は実行時にチェックする

        // セルとそれを含むすべての型はスレッド安全ではない
        // そのため、複数のスレッドからセルにアクセスすることはできない
    }
}
