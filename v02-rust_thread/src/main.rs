use std::thread::spawn;

fn main() {
    my_func11();
    my_func12();
}

fn hello() {
    println!("Hello World!");
}

fn my_func11() {
    spawn(hello).join();
    let h = || println!("Hello World!");
    spawn(h).join();
}

fn my_func12() {
    let v = 10;
    let f = move || v * 2;

    let result = spawn(f).join();
    println!("result = {:?}", result);

    match spawn(|| panic!("I'm panicked!")).join() {
        Ok(_) => {
            println!("successed");
        }
        Err(a) => {
            let s = a.downcast_ref::<&str>();
            println!("failed: {:?}", s)
        }
    }
}