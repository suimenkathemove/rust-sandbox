fn main() {
    // テストは#[test]属性が付与された通常の関数

    // cargo test fooを実行すると、名前のどこかにfooが含まれているテストだけが実行される

    // {
    //     // エラーが起きる場合をテストしたいなら、#[should_panic]属性を付与する
    //     #[test]
    //     #[should_panic(expected = "divide by zero")]
    //     fn test_divide_by_zero_error() {
    //         1 / 0;
    //     }
    // }

    {
        // fn roughly_equal(a: f64, b: f64) -> bool {
        //     (a - b).abs() < 1e-6
        // }

        // #[test]
        // fn trig_works() {
        //     use std::f64::consts::PI;
        //     assert!(roughly_equal(PI.sin(), 0.0));
        // }

        // テストがサポート用のコードを必要とするほど大きくなってきたら、
        // testsモジュールを作って、モジュール全体をテスト時にしかコンパイルされないように、#[cfg]属性で設定するのが慣例となっている

        #[cfg(test)]
        mod tests {
            fn roughly_equal(a: f64, b: f64) -> bool {
                (a - b).abs() < 1e-6
            }

            #[test]
            fn trig_works() {
                use std::f64::consts::PI;
                assert!(roughly_equal(PI.sin(), 0.0));
            }
        }
    }
}
