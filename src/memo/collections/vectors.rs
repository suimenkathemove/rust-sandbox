fn main() {
    // イテレータをベクタに変換
    let v: Vec<i32> = (0..5).collect();

    // 必要とする要素数が事前に分かっているなら、Vec::with_capacityを用いると、要素を追加してもバッファの再確保は生じない
    // vec!マクロは想定よりも多くの要素が必要になったら、自動的にバッファが拡張される

    let mut v: Vec<i32> = Vec::with_capacity(2);
    assert_eq!(v.len(), 0);
    assert_eq!(v.capacity(), 2);
}
