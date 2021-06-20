fn main() {
    {
        // file!, line!, column

        println!("{}", file!());
        println!("{}", line!());
        println!("{}", column!());

        // このマクロを呼び出したマクロを呼び出した、、最初のマクロが呼び出された場所を指す
    }

    {
        // stringify!

        // 引数に与えたトークン列を含んだ文字列リテラルに展開される

        // 引数内のマクロ呼び出しは展開されない

        assert_eq!(stringify!(file!()), "file ! ()");
    }

    {
        // concat!

        assert_eq!(concat!(0, 1, 2), "012");
    }

    {
        // cfg

        // 現在のビルド設定が括弧内の条件にマッチしたかどうかを表す真偽値定数に展開される
    }

    {
        // env!, option_env!

        assert_eq!(env!("USER"), "katouyoshiharu");
        assert_eq!(option_env!("USER"), Some("katouyoshiharu"));
    }

    {
        // include!, include_str!, include_bytes!

        // 指定したファイルの中身に展開される
    }
}
