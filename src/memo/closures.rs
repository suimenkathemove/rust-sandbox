use std::{collections::HashMap, vec};

fn main() {
    struct City {
        name: String,
        population: i64,
        country: String,
    }

    fn sort_cities(cities: &mut Vec<City>) {
        cities.sort_by_key(|c| -c.population);
    }

    {
        // 変数のキャプチャ

        {
            // 借用するクロージャ
        }

        {
            // 盗むクロージャ

            use std::thread;

            struct City {
                name: String,
                population: i64,
                country: String,
            }

            struct Statistic {}

            impl City {
                fn get_statistic(self, stat: Statistic) -> i64 {
                    0
                }
            }

            // fn start_sorting_thread(
            //     mut cities: Vec<City>,
            //     stat: Statistic,
            // ) -> thread::JoinHandle<Vec<City>> {
            //     let key_fn = move |c: &City| -> i64 { -c.get_statistic(stat) };

            //     thread::spawn(move || {
            //         cities.sort_by_key(key_fn);
            //         cities
            //     })
            // }
        }
    }

    {
        // 関数型とクロージャ型

        fn foo(f: fn()) {};
        fn foo2<F>(f: F)
        where
            F: Fn(),
        {
        };
        fn void() {};

        foo(void);
        foo(|| {});

        foo2(void);
        foo2(|| {});
    }

    {
        // クロージャの性能

        // 高速でコンパクトで安全
    }

    {
        // クロージャと安全性

        {
            // 殺すクロージャ

            let str = "hello".to_string();
            let f = || drop(str);

            f();
            // f(); // error: use of moved value
        }

        {
            // FnOnce

            fn call_twice<F>(closure: F)
            where
                F: Fn(),
            {
                closure();
                closure();
            }

            let str = "hello".to_string();
            let f = || drop(str);
            // call_twice(f);

            // trait Fn() -> R {
            //     fn call(&self) -> R;
            // }

            // trait FnOnce() -> R {
            //     fn call_once(self) -> R;
            // }
        }

        {
            // FnMut

            // trait FnMut() -> R {
            //     fn call_mut(&mut self) -> R;
            // }

            // 何らかの値にmutアクセスするが、値のドロップはしないクロージャは、FnMutクロージャとなる

            fn call_twice<F>(mut closure: F)
            where
                F: FnMut(),
            {
                closure();
                closure();
            }

            let mut i = 0;
            let incr = || {
                i += 1;
            };
            call_twice(incr);

            // Fnに属するクロージャはすべてFnMutの要求を満たし、FnMutに属するクロージャはすべてFnOnceの要求を満たす
        }
    }

    {
        // コールバック

        struct Request {
            method: String,
            url: String,
            headers: HashMap<String, String>,
            body: Vec<u8>,
        }

        struct Response {
            code: u32,
            headers: HashMap<String, String>,
            body: Vec<u8>,
        }

        struct BasicRouter<C>
        where
            C: Fn(&Request) -> Response,
        {
            routes: HashMap<String, C>,
        }

        impl<C> BasicRouter<C>
        where
            C: Fn(&Request) -> Response,
        {
            fn new() -> BasicRouter<C> {
                BasicRouter {
                    routes: HashMap::new(),
                }
            }

            fn add_route(&mut self, url: &str, callback: C) {
                self.routes.insert(url.to_string(), callback);
            }
        }

        fn callback(req: &Request) -> Response {
            Response {
                code: 200,
                headers: HashMap::new(),
                body: vec![],
            }
        };

        let mut router = BasicRouter::new();
        router.add_route("/", |req| callback(req));
        // router.add_route("/", |req| callback(req)); // error

        // さまざまな型をサポートしたい場合は、ボックスとトレイトオブジェクトを使う必要がある
        // 個々のボックスには異なる型のクロージャが入れられるので、1つのHashMapがさまざまなコールバックを格納できる
        type BoxedCallback = Box<Fn(&Request) -> Response>;

        struct BasicRouter2 {
            routes: HashMap<String, BoxedCallback>,
        }

        // スコープからすぐに外れてしまうような変数への参照の借用を格納したクロージャを保持するのは安全ではないので、'staticライフタイムを指定する
        impl BasicRouter2 {
            fn new() -> BasicRouter2 {
                BasicRouter2 {
                    routes: HashMap::new(),
                }
            }

            fn add_route<C>(&mut self, url: &str, callback: C)
            where
                C: Fn(&Request) -> Response + 'static,
            {
                self.routes.insert(url.to_string(), Box::new(callback));
            }

            fn handle_request(&self, request: &Request) -> Response {
                match self.routes.get(&request.url) {
                    None => Response {
                        code: 404,
                        headers: HashMap::new(),
                        body: vec![],
                    },
                    Some(callback) => callback(request),
                }
            }
        }
    }
}
