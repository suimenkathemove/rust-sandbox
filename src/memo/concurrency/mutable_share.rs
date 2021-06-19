fn main() {
    {
        // Mutex

        use std::sync::{Arc, Mutex};

        type PlayerId = u32;

        const GAME_SIZE: usize = 8;

        type WaitingList = Vec<PlayerId>;

        struct FernEmpireApp {
            waiting_list: Mutex<WaitingList>,
        }

        let app = Arc::new(FernEmpireApp {
            waiting_list: Mutex::new(vec![]),
        });

        // Arcはスレッド間で何かを共有する際に有用で、Mutexは複数のスレッドから共有アクセスされる可変データを表すのに便利である

        impl FernEmpireApp {
            fn join_waiting_list(&self, player: PlayerId) {
                let mut guard = self.waiting_list.lock().unwrap();

                guard.push(player);
                if guard.len() == GAME_SIZE {
                    let players = guard.split_off(0);
                    // self.start_game(players);
                }
            }
        }

        // Mutexそのものに対しては複数のスレッドが共有アクセス権限を持っている場合でも、内部データに対しては排他アクセスを提供することができる

        {
            // 排他ロックを使うスレッドの問題

            // ・プログラムの動作がスレッドのタイミングに依存し、実行する度に違う結果が出る
            // ・結合度が高い設計になりやすい
            // ・デッドロック
            // ・毒された排他ロック
        }

        {
            // デッドロック

            // ・すでに保有しているロックを再度取りに行く場合
            // ・複数のスレッドが複数の排他ロックを同時に取ろうとする場合

            // クリティカルセクションを小さくすることで防ぐ
        }

        {
            // 毒された排他ロック

            // Mutexを保持したスレッドがパニックを起こすと、そのMutexは毒されたものとしてマークされる
            // 毒されたMutexへのそれ以降のlockはエラーとなる
        }

        {
            // 排他ロックを用いた、複数の消費者を持つチャネル

            // チャネルのReceiverは1つしか作れない

            // MutexでReceiverをくるむと、共有することができる

            pub mod shared_channel {
                use std::sync::mpsc::{channel, Receiver, Sender};
                use std::sync::{Arc, Mutex};

                #[derive(Clone)]
                pub struct SharedReceiver<T>(Arc<Mutex<Receiver<T>>>);

                impl<T> Iterator for SharedReceiver<T> {
                    type Item = T;

                    fn next(&mut self) -> Option<T> {
                        let guard = self.0.lock().unwrap();
                        guard.recv().ok()
                    }
                }

                pub fn shared_channel<T>() -> (Sender<T>, SharedReceiver<T>) {
                    let (sender, receiver) = channel();
                    (sender, SharedReceiver(Arc::new(Mutex::new(receiver))))
                }
            }
        }
    }

    {
        // RwLock

        use std::sync::RwLock;

        struct AppConfig {
            mushrooms_enabled: bool,
        }

        struct FernEmpireApp {
            config: RwLock<AppConfig>,
        }

        impl FernEmpireApp {
            fn mushrooms_enabled(&self) -> bool {
                let config_guard = self.config.read().unwrap();
                config_guard.mushrooms_enabled
            }

            fn reload_config(&self) {
                let new_config = AppConfig {
                    mushrooms_enabled: false,
                };
                let mut config_guard = self.config.write().unwrap();
                *config_guard = new_config;
            }
        }
    }

    {
        // Condvar

        // waitは、別のスレッドがnotify_allを呼ぶまでブロックする

        // 条件変数は、特定のMutexで守られたデータに関する、特定の真偽状態に対するもの
    }

    {
        // アトミック変数

        // 複数のスレッドが同時に読み書きしてもデータ競合が発生しない

        // 通常の数値演算子や論理演算子ではなく、ロード、ストア、交換、数値演算などの、アトミックな操作を行うメソッドを提供している

        {
            use std::sync::atomic::{AtomicIsize, Ordering};

            let atom = AtomicIsize::new(0);
            atom.fetch_add(1, Ordering::SeqCst);
        }

        {
            use std::sync::{
                atomic::{AtomicBool, Ordering},
                Arc,
            };
            use std::thread::spawn;

            let cancel_flag = Arc::new(AtomicBool::new(false));
            let worker_cancel_flag = cancel_flag.clone();

            let worker_handle = spawn(move || {
                if worker_cancel_flag.load(Ordering::SeqCst) {
                    return None;
                }
                Some(())
            });

            // メインスレッドでワーカスレッドをキャンセルする
            cancel_flag.store(true, Ordering::SeqCst);

            worker_handle.join().unwrap();

            // Mutexでもチャネルでも実装できるが、アトミックのオーバーヘッドが最小である

            // アトミックはselfを共有参照として受け取るため、単純なグローバル変数の実装にも使える
        }
    }

    {
        // グローバル変数

        // static PACKET_SERVED: usize = 0;

        // スレッド安全であり、かつ可変なstatic変数を宣言する一番簡単な方法は、アトミック変数にすること

        use std::sync::atomic::{AtomicUsize, Ordering};

        static PACKET_SERVED: AtomicUsize = AtomicUsize::new(0);
        PACKET_SERVED.fetch_add(1, Ordering::SeqCst);

        // グローバルな他の型の変数を作るためには、

        // ・変数はスレッド安全になっていなければならない
        // Syncで非mutでなければstatic変数を作ることはできない
        // Mutex

        use std::sync::Mutex;

        // static FOO: Mutex<String> = Mutex::new(String::new()); // error: function call in static

        use lazy_static::lazy_static;

        lazy_static! {
            static ref FOO: Mutex<String> = Mutex::new(String::new());
        }

        // lazy_static!を使うと、staticデータにアクセスする度に性能上のコストが少しずつかかる
    }
}
