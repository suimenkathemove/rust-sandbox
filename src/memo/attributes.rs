fn main() {
    // attributeは、コンパイラへのさまざまな命令やアドバイスを書くための構文
    // どのアイテムにも属性を付加することができる

    #[allow(non_camel_case_types)]
    struct foo_bar {}

    // #[cfg]や#[allow]などの一部のattributeは、モジュール全体に付与してその中のアイテムすべてに適用することもできる

    // attributeをクレート全体に付与するためには、main.rsかlib.rsの先頭に書く

    // #!は、その行を包含するアイテムに対して付与することを意味する
}
