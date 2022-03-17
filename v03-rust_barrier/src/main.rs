use std::sync::{Arc, Barrier};
use std::thread;

fn main() {
    // スレッドハンドラを保存するベクタ
    let mut v = Vec::new();

    // 10スレッド分のバリア同期をArcで包む
    let barrier = Arc::new(Barrier::new(10));

    // 10スレッド起動
    for _ in 0..20 {
        let b = barrier.clone();
        let th = thread::spawn(move || {
            b.wait(); //バリア同期
            println!("finished barrier");
        });
        v.push(th);
    }

    for th in v {
        th.join().unwrap();
    }
}
