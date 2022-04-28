use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let val = Arc::new(RwLock::new(true));

    let t = thread::spawn(move || {
        // deadlock
        // let flag = val.read().unwrap();
        // if *flag {
        //     *val.write().unwrap() = false;
        //     println!("flag is true");
        // }

        // lockを即時捨てる deadlockにならない
        // let flag = *val.read().unwrap();
        // if flag {
        //     *val.write().unwrap() = false;
        //     println!("flag is true");
        // }

        // deadlock
        // let _flag = val.read().unwrap();
        // *val.write().unwrap() = false;
        // println!("deadlock");

        let _ = val.read().unwrap();
        *val.write().unwrap() = false;
        println!("not deadlock");
    });

    t.join().unwrap();
}
