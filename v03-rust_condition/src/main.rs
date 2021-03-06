use std::sync::{Arc, Condvar, Mutex};
use std::thread;

fn main() {
    let pair0 = Arc::new((Mutex::new(false), Condvar::new()));
    let pair1 = pair0.clone();
    let pair2 = pair0.clone();

    let c0 = thread::spawn(move || child(0, pair0));
    let c1 = thread::spawn(move || child(1, pair1));

    let p = thread::spawn(move || parent(pair2));

    c0.join().unwrap();
    c1.join().unwrap();
    p.join().unwrap();
}

// Condvar型の変数が条件変数であり、MutexとCondvarを含むタプルがArcに包んで渡される
fn child(id: u64, p: Arc<(Mutex<bool>, Condvar)>) {
    let &(ref lock, ref cvar) = &*p;

    // まずmutexロックを行う
    let mut started = lock.lock().unwrap();
    while !*started {
        // waitで待機
        started = cvar.wait(started).unwrap();
    }

    // 以下のようにwait_whileを使うことも可能
    // cvar.wait_while(started, |started| !*started).unwrap();

    println!("child {}", id);
}

fn parent(p: Arc<(Mutex<bool>, Condvar)>) {
    let &(ref lock, ref cvar) = &*p;

    // まずmutexロックを行う
    let mut started = lock.lock().unwrap();
    *started = true; // 共有変数を更新
    cvar.notify_all(); // 通知
    println!("parent");
}
