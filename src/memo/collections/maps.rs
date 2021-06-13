use std::collections::{BTreeMap, HashMap};

fn main() {
    {
        // HashMap

        // キーと値をハッシュテーブルに保持する
        // キーの型は、ハッシュトレイトであるHashと、等価判定トレイトであるEqを実装していなければならない
    }

    {
        // BTreeMap

        // エントリをキーの順番で木構造に保持する
        // そのため、キーの型はOrdを実装していなければならない
    }

    let mut hm1 = HashMap::new();
    hm1.insert("one", 1);

    let hm2 = vec![("one", 1)].into_iter().collect::<HashMap<_, _>>();

    assert_eq!(hm2, hm1);

    let mut btm1 = BTreeMap::new();
    btm1.insert("one", 1);

    let btm2 = vec![("one", 1)].into_iter().collect::<BTreeMap<_, _>>();

    assert_eq!(btm2, btm1);

    // HashMapはVecと同様に、データを1つのヒープ上の領域に格納する
    // これにより、容量に関連したメソッドである、with_capacity、capacity、reserve、shrink_to_fitを持つ

    {
        // len
    }

    {
        // is_empty
    }

    {
        // contains_key
    }

    {
        // get, get_mut

        // キーに対してmutアクセスは許さない
    }

    {
        // insert

        let mut hm = HashMap::new();

        assert_eq!(hm.insert("one", 1), None);
        assert_eq!(hm.insert("one", 1), Some(1));
    }

    {
        // extend
    }

    {
        // append
    }

    {
        // remove
    }

    {
        // clear
    }

    let mut hm = HashMap::new();
    hm.insert("one".to_string(), 1);
    assert_eq!(hm.get("one"), Some(&1));

    {
        // split_at
    }

    {
        // エントリ

        // エントリの目的は、冗長なマップ検索を削除すること
        // 一度の検索でEntry値を生成させ、後の処理はそれに対して行うことで、検索の回数を減らすことができる

        struct User<'a> {
            name: &'a str,
            age: i32,
        }

        let mut users: HashMap<&str, User> = HashMap::new();

        users.insert(
            "foo",
            User {
                name: "foo",
                age: 0,
            },
        );

        let name = "foo";

        if !users.contains_key(name) {
            users.insert(name, User { name, age: 0 });
        }
        let _foo = users.get_mut(name).unwrap();

        // 一度しか検索しない
        let _foo = users.entry(name).or_insert(User { name, age: 0 });
        let _foo = users.entry(name).or_insert_with(|| User { name, age: 0 });

        // entryが返したEntry値は、マップ中のキーバーリューペアが占有している場所か、エントリがまだ無い空き地への可変参照のように振る舞う

        // すべてのEntry値は、同じメソッドで作られる

        {
            // entry

            // 指定されたkeyに対するEntryを返す
            // 指定されたkeyがマップ中に無ければ、空のEntryを指す

            // pub fn entry<'a>(&'a mut self, key: K) -> Entry<'a, K, V>
            // 'aがあるのはマップから借用するmut参照の一種だから
            // Entryは、生存している限りマップに対して排他的なアクセス権を持つ

            // マップのキー型がStringの場合に、このメソッドに&strを与えることはできない
        }
    }

    {
        // イテレート

        // HashMapのイテレータは、マップ中のエントリを予測できない順番で訪れる
        // BTreeMapのイテレータは、キーの順番通りに訪れる
    }
}
