use std::{collections::HashMap, ptr::eq};

fn main() {
    // 所有者であるポインタがドロップされると参照先もドロップされる
    // 参照　所有権を持たないポインタ型
    // 参照先のライフタイムに何の影響も持たない
    // 参照先よりも長生きしてはいけない

    // 借用　ある値に対して参照を作ること

    type Table = HashMap<String, Vec<String>>;
    fn show(table: Table) {
        for (artist, works) in table {
            println!("{}", artist);
            for work in works {
                println!("{}", work);
            }
        }
    }
    let mut table = Table::new();
    table.insert(
        "foo".to_string(),
        vec!["a".to_string(), "b".to_string(), "c".to_string()],
    );
    show(table);
    // println!("{}", table["foo"][0]); // value borrowed here after move

    // 共有参照　複数読み出し
    // 可変参照　単一書き込み　排他的なアクセス権を持ち、その間は所有者を使うことはできない
    // 他の参照とは同時に使用することはできない　Copy型ではない
    let mut foo = "foo".to_string();
    let bar = &mut foo;
    // let baz = foo; // first borrow later used here
    // println!("{} {}", bar, baz);

    fn show2(table: &Table) {
        // vectorやHashMapの共有参照に対する繰り返し実行をする場合、カウンタ変数も共有参照になる
        for (artist, works) in table {
            println!("{}", artist);
            // ここも
            for work in works {
                println!("{}", work);
            }
        }
    }

    // .演算子が、必要に応じて暗黙に左のオペランドを参照解決するようになっている
    // .演算子は、メソッド呼び出しの際に、必要があれば暗黙に左オペランドへの参照を借用する
    struct User {
        name: &'static str,
    };
    let user = User { name: "John Doe" };
    let user_ref = &user;
    println!("{}", user_ref.name);
    println!("{}", (*user_ref).name);

    // .演算子と同様に、比較演算子も参照解決することができる
    let x = 10;
    let y = 10;
    let rx = &x;
    let ry = &y;
    let rrx = &rx;
    let rry = &ry;
    assert!(rrx == rry);
    assert!(!eq(rrx, rry));

    // 初期化されるまで変数を使うことはできない
    // 整数から参照へ変換することはできない
    // 機械語レベルではNoneはヌルポインタ　OptionはNoneかどうかを確認しないと使えないから安全である

    // 任意の式の値に対して参照を借用することができる
    fn factorial(n: usize) -> usize {
        (1..=n).fold(1, |a, b| a * b)
    }
    let r = &factorial(6);
    assert_eq!(r + &1, 721);
    // 式の値を保持する無名の変数を作り、参照がそれを指すようにする
    // ・参照をlet文ですぐに変数に代入する場合（もしくは構造体や配列などの値の一部としてすぐに代入する場合）、無名変数のライフタイムは、letが初期化した変数のライフタイムと同じになる
    // ・それ以外の場合には、無名変数のライフタイムはそれをくるんでいる文の終わりまでになる

    // ファットポインタ⋯何らかの値へのアドレスと、その値を使うために必要な情報を持つワードの2ワードで構成される
    // ・スライスへの参照
    // ・トレイトオブジェクトへの参照

    // ローカル変数の参照を借用して、その変数のスコープの外に持ち出すことはできない
    // Rustコンパイラは、プログラム中のすべての参照型に対して、その参照の使われ方によって生じる制約を反映したライフタイムを割り当てる
    // 参照のライフタイムは型の一部
    // {
    //     // rのライフタイム
    //     let r;
    //     {
    //         // xのライフタイム
    //         let x = 1;
    //         // xに対して借用し、rに格納した参照型のライフタイム
    //         r = &x; // borrowed value does not live long enough
    //     }
    //     // borrowed value does not live long enough
    //     // requires that `x` is borrowed for `'static`
    //     assert_eq!(*r, 1);
    // }
    // ある参照を変数に格納する場合、その参照型の値は、その変数のライフタイム全体で有効でなければならない
    // プログラム中での参照の使われ方によって生じる制約を理解し、その制約を満たすライフタイムを見つける

    static mut STASH: &i32 = &128;
    // fn f(p: &i32) {
    fn f<'a>(p: &'a i32) {
        unsafe {
            // STASHはプログラム実行全体をライフタイムとするので、この変数が保持する参照型も同じ長さのライフタイムを持っていなければならない
            // pの参照のライフタイムは'a、すなわちfの呼び出し部分を包含する任意の期間
            // STASH = p; // lifetime of reference outlives lifetime of borrowed content...
        }
    }
    // 'a（tickAと発音する）⋯ライフタイムパラメータ
    static mut STASH2: &i32 = &10;
    fn f2(p: &'static i32) {
        unsafe {
            STASH2 = p;
        }
    }
    // 関数のシグネチャを見るだけで、その引数に対してできることとできないことがわかる

    // ライフタイムパラメータは関数や型を定義するときだけ指定する

    // fn smallest(v: &[i32]) -> &i32 {
    fn smallest<'a>(v: &'a [i32]) -> &'a i32 {
        let mut s = &v[0];
        for r in &v[1..] {
            if *r < *s {
                s = r;
            }
        }
        s
    }
    // 関数シグネチャにライフタイムの情報を持つことによって、関数に渡す参照と関数が返す参照の関係を明示することができる

    // struct S {
    //     r: &i32, // expected named lifetime parameter
    // }
    // let s;
    // {
    //     let x = 10;
    //     s = S { r: &x };
    // }
    // assert_eq!(*s.r, 10);
    // 参照型が他の型の定義に含まれている場合、そのライフタイムを明示的に書かなければならない
    struct S2<'a> {
        r: &'a i32,
    }
    // S2型は参照型同様にライフタイムを持つようになる
    // rに保持される参照のライフタイムは'aを包含していなければならず、'aはS2を収めた何かのライフタイムを包含していなければならない

    // ライフタイムパラメータが指定された型を、他の型に内包した場合
    struct T {
        s: S2<'static>,
    }
    struct T2<'a> {
        s: S2<'a>,
    }

    // 全ての型がライフタイムを持つ

    {
        struct S<'a> {
            x: &'a i32,
            y: &'a i32,
        }
        let x = 10;
        let r;
        {
            let y = 20;
            {
                // yは'aを包含しなければならない
                let s = S { x: &x, y: &y };
                // 'aはrを包含しなければならない
                r = s.x
            }
        }
        // yのスコープよりも短く、rのスコープより長いライフタイムは無い
    }

    {
        // 関数が参照（もしくは何らかのライフタイムパラメータを必要とする型）を返さない場合は、仮引数に対してもライフタイムを省略することができる
        struct S<'a, 'b> {
            x: &'a i32,
            y: &'b i32,
        }
        // fn sum_r_xy<'a, 'b, 'c>(r: &'a i32, s: S<'b, 'c>) -> i32 {
        fn sum_r_xy(r: &i32, s: S) -> i32 {
            r + s.x + s.y
        }

        // 参照などの、ライフタイムパラメータを持つ型を返す場合でも、曖昧さが無い場合には簡単に書けるようになっている
        // 関数の仮引数にライフタイムが1つしかない場合には、それを返り値に出現する全てのライフタイムと同じだと想定する
        // fn first_third<'a>(point: &'a [i32; 3]) -> (&'a i32, &'a i32) {
        fn first_third(point: &[i32; 3]) -> (&i32, &i32) {
            (&point[0], &point[2])
        }
        // 仮引数に複数のライフタイムが現れる場合は、明示的に書く必要がある
        // メソッドで、self仮引数を参照で取る場合には、selfのライフタイムを、返り値に存在する全てのライフタイムにする
    }

    // {
    //     // 共有参照はライフタイムの間中のみ、参照先を読み出し可能な状態にする
    //     let v = vec![4, 8, 19, 27, 34, 10];
    //     let r = &v;
    //     let aside = v; // cannot move out of `v` because it is borrowed
    //     r[0];
    // }
    {
        let v = vec![4, 8, 19, 27, 34, 10];
        {
            let r = &v;
            r[0];
        }
        let aside = v;
    }

    {
        fn extend(vec: &mut Vec<f64>, slice: &[f64]) {
            for elt in slice {
                vec.push(*elt);
            }
        }
        let mut wave = Vec::new();
        let head = vec![0., 1.];
        let tail = [0., -1.];
        extend(&mut wave, &head);
        extend(&mut wave, &tail);
        assert_eq!(wave, vec![0., 1., 0., -1.]);
        // cannot borrow `wave` as immutable because it is also borrowed as mutable
        // extend(&mut wave, &wave);
    }
}
