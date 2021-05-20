fn main() {
    // as演算子でbool値を整数型に変換できる
    assert_eq!(false as i32, 0);
    assert_eq!(true as i32, 1);

    // 数値をboolにすることはできない
    assert_eq!(0 as bool, false); // error

    // bool値に対するポインタを作れるようにするために、Rustではboolを表現するのに1バイトを使う
}
