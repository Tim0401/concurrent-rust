
// channel.rs
use crate::semaphore::Semaphore;
use std::collections::LinkedList;
use std::sync::{Arc, Condvar, Mutex};

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

// 受信端のための型
pub struct Receiver<T> {
    sem: Arc<Semaphore>,            // 有限性を実現するセマフォ
    buf: Arc<Mutex<LinkedList<T>>>, // キュー
    cond: Arc<Condvar>,
}

impl<T> Receiver<T> {
    pub fn recv(&self) -> T {
        let mut buf = self.buf.lock().unwrap();
        loop {
            // キューから取り出し
            if let Some(data) = buf.pop_front() {
                self.sem.post();
                return data;
            }
            buf = self.cond.wait(buf).unwrap();
        }
    }
}

pub fn channel<T>(max: isize) -> (Sender<T>, Receiver<T>) {
    assert!(max > 0);
    let sem = Arc::new(Semaphore::new(max));
    let buf = Arc::new(Mutex::new(LinkedList::new()));
    let cond = Arc::new(Condvar::new());
    let tx = Sender {
        sem: sem.clone(),
        buf: buf.clone(),
        cond: cond.clone(),
    };
    let rx = Receiver { sem, buf, cond };
    (tx, rx)
}
