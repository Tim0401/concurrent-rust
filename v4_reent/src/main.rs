use std::{
    borrow::BorrowMut,
    sync::{Arc, Mutex},
};

fn main() {
    // ミューテックスをArcで作成してクローン
    let lock0 = Arc::new(Mutex::new(0)); // <1>
                                         // Arcのクローンは参照カウンタを増やすだけ
    let lock1 = lock0.clone(); // <2>

    let a = lock0.lock().unwrap();
    // TODO aを更新するには？
    println!("{}", a);

    let b = lock1.lock().unwrap(); // デッドロック <3>
    println!("{}", b);
}
