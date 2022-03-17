use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Arcはスレッドセーフな参照カウンタ型のスマートポインタ
    let lock0 = Arc::new(Mutex::new(0));
    // 参照カウンタがインクリメントされるのみで中身はクローンされない
    let lock1 = lock0.clone();

    // スレッド生成
    // クロージャー内変数へmove
    let th0 = thread::spawn(move || {
        some_func(lock0);
    });

    let th1 = thread::spawn(move || {
        some_func(lock1);
    });

    th0.join().unwrap();
    th1.join().unwrap();
}

fn some_func(lock: Arc<Mutex<u64>>) {
    loop {
        // ロックしないとMutex型の中の値は参照不可
        let mut val = lock.lock().unwrap();
        *val += 1;
        println!("{}", *val);
    }
}
