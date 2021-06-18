fn main() {
    // 仕事を、全てのスレッドに均等に分散できない場合がある

    // 各スレッドで計算した結果を統合する時間が必要

    {
        // spawnとjoin

        use std::sync::Arc;
        use std::thread::spawn;

        fn _parallel(v: Arc<Vec<i32>>) {
            let work_lists = (0..4).collect::<Vec<_>>();

            let mut thread_handles = Vec::new();

            for work_list in work_lists {
                let cloned_v = v.clone();
                thread_handles.push(spawn(move || {
                    println!("{}, {:?}", work_list, &cloned_v);
                }));
            }

            for handle in thread_handles {
                handle.join().unwrap();
            }
        }

        // _parallel(Arc::new(vec![]));
    }

    {
        // スレッド間でのエラー処理

        // join

        {
            // 子スレッドがパニックを起こした場合、エラーとなる

            // パニックはスレッド単位で起こる
            // パニックが、あるスレッドに依存する別のスレッドに自動的に広がっていくようなことはない
            // あるスレッドでのパニックは、他のスレッドにエラーのResultとして報告される
        }

        {
            // 子スレッドが返す値を親スレッドに返す
        }
    }

    {
        // Rayon

        // Crossbeamと併せて、フォークジョイン並列をサポートする

        use rayon::par_iter::{IntoParallelRefIterator, ParallelIterator};

        let v = (0..3).collect::<Vec<_>>();

        v.par_iter().for_each(|_n| {
            println!("{}", _n);
        });

        // Rayonがスレッドを管理し、仕事を最良の方法で分散してくれる

        fn _parallel(v1: &[i32]) -> Option<()> {
            v1.par_iter()
                .map(|n| println!("{}", n))
                .reduce_with(|_r1, _r2| {})
        }

        _parallel(&(0..3).collect::<Vec<_>>());
    }
}
