use std::collections::HashMap;

fn main() {
    {
        // Cスタイルの列挙型

        enum Pet {
            Orca,
            Giraffe,
        }

        enum HttpStatus {
            Ok = 200,
            NotModified = 304,
            NotFound = 404,
        }

        // デフォルトでは、組み込みの整数型の中で値を表現できる最もサイズの小さい型を用いて列挙型を表現する

        // 列挙型から整数型へのキャストはできるが、逆はできない
        assert_eq!(HttpStatus::Ok as i32, 200);

        // 自分でチェックしながら変換することはできる
        fn http_status_from(n: u32) -> Option<HttpStatus> {
            match n {
                200 => Some(HttpStatus::Ok),
                304 => Some(HttpStatus::NotModified),
                404 => Some(HttpStatus::NotFound),
                _ => None,
            }
        }

        // または、enum_primitiveクレートを使う

        // 構造体と同様に、==演算子などの機能をコンパイラが実装してくれるが、それには明示的に要求する必要がある
        #[derive(Copy, Clone, Debug, PartialEq)]
        enum TimeUnit {
            Seconds,
            Minutes,
            Hours,
            Days,
            Months,
            Years,
        }

        // 構造体と同様にメソッドを持つこともできる
    }

    {
        // データを保持する列挙型

        #[derive(Copy, Clone, Debug, PartialEq)]
        enum TimeUnit {
            Seconds,
            Minutes,
            Hours,
            Days,
            Months,
            Years,
        }

        #[derive(Copy, Clone, Debug, PartialEq)]
        enum RoughTime {
            InThePast(TimeUnit, u32), // tuple variants
            JustNow,
            InTheFuture(TimeUnit, u32), // tuple variants
        }

        let three_hours_from_now = RoughTime::InTheFuture(TimeUnit::Hours, 3);

        // 構造体型ヴァリアント

        // パブリックな列挙型のコンストラクタとフィールドはすべて、自動的にパブリックになる
    }

    {
        // 列挙型を用いたリッチなデータ構造

        #[derive(Clone, PartialEq, Debug)]
        enum Json {
            Null,
            Boolean(bool),
            Number(f64),
            String(String),
            Array(Vec<Json>),
            Object(Box<HashMap<String, Json>>),
        }

        // メモリ上では、Json型は4ワードを占める
        // String値とVec値が3ワード、さらにタグのバイト
        // NullやBoolean値にはすべてのスペースを埋めるほどのデータはないが、すべてのJson値は同じサイズでなければならないので、余分なスペースは使われないままになる
        // HashMapは、すべてのJson値に十分な領域を保持しようとすると8ワードぐらいになるが、Box<HashMap>はヒープ上に取られたデータへのポインタの1ワードしかない
    }

    {
        // ジェネリック列挙型

        // 型TがBoxなどのスマートポインタ型だった場合には、Option<T>のタグフィールドを削除できる
        // Option<Box<i32>>はメモリ上では1ワードとなる　0がNoneを表し、それ以外はSomeでボックスされた値を表す

        enum BinaryTree<T> {
            Empty,
            NonEmpty(Box<TreeNode<T>>),
        }

        struct TreeNode<T> {
            element: T,
            left: BinaryTree<T>,
            right: BinaryTree<T>,
        }

        let jupiter_tree = BinaryTree::NonEmpty(Box::new(TreeNode {
            element: "Jupiter",
            left: BinaryTree::Empty,
            right: BinaryTree::Empty,
        }));

        // 列挙型の中のデータにアクセスする方法はパターンマッチしかない
    }
}
