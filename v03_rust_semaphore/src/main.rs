use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

const NUM_LOOP: usize = 100000;
const NUM_THREAD: usize = 8;
const SEM_NUM: isize = 4;

static mut CNT: AtomicUsize = AtomicUsize::new(0);

fn main() {
    let mut v = Vec::new();
    // SEM_NUMだけ同時に実行可能なセマフォ
    let sem = Arc::new(Semaphore::new(SEM_NUM));

    for thread_i in 0..NUM_THREAD {
        let s = sem.clone();
        let t = std::thread::spawn(move || {
            for loop_i in 0..NUM_LOOP {
                s.wait();

                // アトミックにインクリメントとデクリメント
                unsafe { CNT.fetch_add(1, Ordering::SeqCst) };
                let n = unsafe { CNT.load(Ordering::SeqCst) };
                println!(
                    "semaphore: thread = {}, loop = {}, CNT = {}",
                    thread_i, loop_i, n
                );
                assert!((n as isize) <= SEM_NUM);
                unsafe { CNT.fetch_sub(1, Ordering::SeqCst) };

                s.post();
            }
        });
        v.push(t);
    }

    for t in v {
        t.join().unwrap();
    }
}

// semaphore.rs

use std::sync::{Condvar, Mutex};
// セマフォ用の型
pub struct Semaphore {
    mutex: Mutex<isize>,
    cond: Condvar,
    max: isize,
}

impl Semaphore {
    pub fn new(max: isize) -> Self {
        Semaphore {
            mutex: Mutex::new(0),
            cond: Condvar::new(),
            max,
        }
    }

    pub fn wait(&self) {
        // カウントが最大値以上なら待機
        let mut cnt = self.mutex.lock().unwrap();
        while *cnt >= self.max {
            cnt = self.cond.wait(cnt).unwrap();
        }
        *cnt += 1;
    }

    pub fn post(&self) {
        // カウントをデクリメント
        let mut cnt = self.mutex.lock().unwrap();
        *cnt -= 1;
        if *cnt <= self.max {
            self.cond.notify_one();
        }
    }
}

// channel.rs
use std::collections::LinkedList;
// use std::sync::{Arc, Condvar, Mutex};

// 送信端のための型
#[derive(Clone)]
pub struct Sender<T> {
    sem: Arc<Semaphore>,            // 有限性を実現するセマフォ
    buf: Arc<Mutex<LinkedList<T>>>, // キュー
    cond: Arc<Condvar>,             // 読み込み側の条件変数
}

impl<T: Send> Sender<T> {
    // 送信関数
    pub fn send(&self, data: T) {
        self.sem.wait(); // キューの最大値に到達したら待機
        let mut buf = self.buf.lock().unwrap();
        buf.push_back(data);
        self.cond.notify_one(); // 読み込み側への通知
    }
}
