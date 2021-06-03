use std::fmt::format;

fn main() {
    {
        enum TimeUnit {
            Seconds,
            Minutes,
            Hours,
            Days,
            Months,
            Years,
        }

        enum RoughTime {
            InThePast(TimeUnit, u32),
            JustNow,
            InTheFuture(TimeUnit, u32),
        }

        fn rough_time_to_english(rt: RoughTime) -> String {
            match rt {
                RoughTime::InThePast(units, count) => format!("{} ago", count),
                RoughTime::JustNow => "just now".to_string(),
                RoughTime::InTheFuture(units, count) => format!("{} from now", count),
            }
        }

        // 式は値を生成し、パターンは値を消費する
        // この2つには多くの場合、同じ構文を使う

        // マッチ対象の値に入っていた値は、新しい変数にコピーもしくは移動される
    }

    {
        // パターン内のリテラル、変数、ワイルドカード

        // ワイルドカードパターンに届かない場合でも、デフォルトの分岐を用意して、パニックを起こすようにする
    }

    {
        // 参照パターン

        // refパターンはマッチした値の一部を借用する
        // &パターンは参照にマッチする

        // 参照へのマッチにはライフタイムが強制される
    }

    {
        // パターンガード

        // ifキーワードを用いて、matchの分岐にガードを付ける事ができる
        // マッチはガード式の評価結果がtrueとなった場合のみ成功する

        // パターンが値を移動する場合には、ガードを付けることはできない
    }

    {
        // @パターン

        // 変数を1つだけ作って、値全体を移動もしくはコピーする
    }

    {
        // 二分木へのデータ追加

        enum BinaryTree<T> {
            Empty,
            NonEmpty(Box<TreeNode<T>>),
        }

        struct TreeNode<T> {
            element: T,
            left: BinaryTree<T>,
            right: BinaryTree<T>,
        }

        impl<T: Ord> BinaryTree<T> {
            fn add(&mut self, value: T) {
                match *self {
                    BinaryTree::Empty => {
                        *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                            element: value,
                            left: BinaryTree::Empty,
                            right: BinaryTree::Empty,
                        }))
                    }
                    BinaryTree::NonEmpty(ref mut node) => {
                        if value <= node.element {
                            node.left.add(value);
                        } else {
                            node.right.add(value);
                        }
                    }
                }
            }
        }

        let mut tree = BinaryTree::Empty;
        tree.add("Mercury");
    }
}
