use std::sync::mpsc::{self, channel, sync_channel, Receiver};
use std::thread::{spawn, JoinHandle};

fn main() {
    // チャネルが空なら、receiver.recv()は、値が送信されるまでブロックする

    fn start_send_thread(nums: Vec<i32>) -> (Receiver<i32>, JoinHandle<()>) {
        let (sender, receiver) = channel();

        let handle = spawn(move || {
            for n in nums {
                if sender.send(n).is_err() {
                    break;
                }
            }
        });

        (receiver, handle)
    }

    fn start_transfer_thread(nums: Receiver<i32>) -> (Receiver<i32>, JoinHandle<()>) {
        let (sender, receiver) = channel();

        let handle = spawn(move || {
            for n in nums {
                let new_n = n * 2;
                if sender.send(new_n).is_err() {
                    break;
                }
            }
        });

        (receiver, handle)
    }

    fn start_receive_thread(nums: Receiver<i32>) {
        while let Ok(n) = nums.recv() {
            println!("{}", n);
        }

        for n in nums {
            println!("{}", n);
        }
    }

    fn run_pipeline(v: Vec<i32>) {
        let (nums1, h1) = start_send_thread(v);

        let (nums2, h2) = start_transfer_thread(nums1);
        let (nums3, h2) = start_transfer_thread(nums2);

        start_receive_thread(nums3);

        h1.join().unwrap();
        h2.join().unwrap();
    }

    run_pipeline((0..3).collect::<Vec<_>>());
    run_pipeline((3..6).collect::<Vec<_>>());

    {
        // 値の送信

        // sendメソッドとrecvメソッドはResultを返すが、失敗するのは、チャネルの相手がドロップされた場合だけ
    }

    {
        // 値の受信

        // 受信スレッドの制御がループの冒頭に移ったときにチャネルが空なら、他のスレッドから値が送られるのを待つ
    }

    {
        // チャネルの機能と性能

        {
            // mpsc⋯multi-producer, single-consumer

            // Sender<T>はCloneトレイトを実装している
            // 複数のSenderを持つチャネルを得るには、普通のチャネルを作って、必要なだけSenderをクローンする
            // それぞれのSenderを別のスレッドに移動することもできる

            // Receiver<T>はCloneできない
            // 1つのチャネルから複数のスレッドで値を受け取るには、Mutexを使う
        }

        {
            // 受信スレッドが処理できるよりも速く値を送信すると、チャネルの性能を低下させてしまう

            // 同期チャネルは、作成時に、保持できる値の数を指定することができる

            let (_sender, _receiver) = sync_channel::<i32>(1000);
        }
    }

    {
        // スレッド安全性

        // Sendを実装する型は、他のスレッドに値で渡しても安全である
        // Syncを実装する型は、他のスレッドに非mut参照で渡しても安全である
    }

    {
        // スレッドのパイプラインをイテレータのパイプラインと統合する

        pub trait OffThreadExt: Iterator {
            fn off_thread(self) -> mpsc::IntoIter<Self::Item>;
        }

        impl<T> OffThreadExt for T
        where
            T: Iterator + Send + 'static,
            T::Item: Send + 'static,
        {
            fn off_thread(self) -> mpsc::IntoIter<Self::Item> {
                let (sender, receiver) = mpsc::sync_channel(1024);

                spawn(move || {
                    for item in self {
                        if sender.send(item).is_err() {
                            break;
                        }
                    }
                });

                receiver.into_iter()
            }
        }

        (0..3)
            .map(|n| n * 2)
            .off_thread()
            .map(|n| n * 2)
            .off_thread()
            .for_each(|n| {
                println!("{}", n);
            });
    }
}
