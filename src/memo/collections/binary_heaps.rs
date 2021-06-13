use std::collections::BinaryHeap;

fn main() {
    // 最大値が常にキューの先頭に浮き上がってくるように緩やかに構造化されたコレクション

    // push

    // pop
    // 最大値を削除して返す

    // peek
    // 最大値の参照を返す

    let mut h = BinaryHeap::from((0..5).collect::<Vec<i32>>());

    assert_eq!(h.peek(), Some(&4));
    assert_eq!(h.pop(), Some(4));
    assert_eq!(h.pop(), Some(3));

    // タスクキューを作るのに適している

    // イテレータは、ヒープの要素を大きい順ではなく、予測できない順番で返す
    // 優先順に値を消費するには、whileループを使う

    while let Some(i) = h.pop() {
        println!("{}", i);
    }
}
