use std::collections::HashMap;

fn main() {
    #[derive(Clone, PartialEq, Debug)]
    enum Json {
        Null,
        Boolean(bool),
        Number(f64),
        String(String),
        Array(Vec<Json>),
        Object(Box<HashMap<String, Json>>),
    }

    impl From<bool> for Json {
        fn from(b: bool) -> Self {
            Json::Boolean(b)
        }
    }

    impl From<String> for Json {
        fn from(s: String) -> Self {
            Json::String(s)
        }
    }

    impl<'a> From<&'a str> for Json {
        fn from(s: &'a str) -> Self {
            Json::String(s.to_string())
        }
    }

    macro_rules! impl_from_num_for_json {
        ( $( $t:ident )* ) => {
            $(
                impl From<$t> for Json {
                    fn from(n: $t) -> Self {
                        Json::Number(n as f64)
                    }
                }
            )*
        }
    }
    impl_from_num_for_json!(u8 i8 u16 i16 u32 i32 u64 i64 usize isize f32 f64);

    {
        // フラグメント型

        macro_rules! json {
            (null) => {
                Json::Null
            };
            ([ $( $elem:tt ),* ]) => {
                Json::Array(vec![ $( json!($elem) ),* ])
            };
            ({ $( $key:tt : $value:tt ),* }) => {
                Json::Object(Box::new(vec![ $( ($key.to_string(), json!($value)) ),* ].into_iter().collect()))
            };
            ($other:tt) => {
                Json::from($other)
            };
        }

        fn json_null() {
            assert_eq!(json!(null), Json::Null);
        }
        json_null();

        fn json_array_with_json_element() {
            let macro_generated_value = json!(
                [
                    {
                        "foo": 0
                    }
                ]
            );

            let hand_coded_value = Json::Array(vec![Json::Object(Box::new(
                vec![("foo".to_string(), Json::Number(0.))]
                    .into_iter()
                    .collect::<HashMap<_, _>>(),
            ))]);

            assert_eq!(macro_generated_value, hand_coded_value);
        }
        json_array_with_json_element();

        {
            // expr⋯式
        }

        {
            // stmt⋯式または宣言

            // 使いにくいので、exprかblockを使った方がよい
        }

        {
            // ty⋯型
        }

        {
            // path⋯パス
        }

        {
            // pat⋯パターン
        }

        {
            // item⋯アイテム
        }

        {
            // block⋯ブロック
        }

        {
            // meta⋯属性のボディ部
        }

        {
            // ident⋯識別子
        }

        {
            // tt⋯トークンツリー

            // ネストしたツリーにもマッチする
        }

        // 拡張することはできない
    }

    {
        // 再帰

        // コンパイラはマクロの再帰呼び出しをデフォルトで64回に制限している

        // 上限を増やしたい場合は、マクロを使うクレートの冒頭に、例えば#[!recursion_limit = "256"]を追加する
    }

    {
        // トレイト

        // マクロは型の判別に適していない

        // 様々な型の値を特定の型に変換するには、Fromトレイトを使う
    }

    {
        // スコープと健全マクロ

        // 健全化はローカル変数と引数だけに限定されている

        macro_rules! foo {
            ($bar:ident, $baz:ident) => {
                let $bar = $baz + 1;
            };
        }

        let baz = 1;
        foo!(bar, baz);
        assert_eq!(bar, 2);
    }

    {
        // インポートとエクスポート

        // あるモジュールで見えているマクロは、自動的にその子モジュールでも見える

        // あるモジュールから親モジュールへマクロをエクスポートするには、#[macro_use]属性を用いる

        // クレート内のマクロをエクスポートするには、パブリックなマクロにそれぞれ#[macro_export]を付与する

        // エクスポートされたマクロは、何かがスコープに入っていることに依存してはいけない
    }
}
