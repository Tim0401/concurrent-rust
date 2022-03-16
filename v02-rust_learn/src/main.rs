fn main() {
    my_func2();
    my_func3();
}

// 参照外し
fn mul(x: &mut i32, y: &i32) {
    *x *= *x * *y;
}

fn my_func2() {
    let mut n = 10;
    let m = 20;
    println!("n = {}, m = {}", n, m);
    mul(&mut n, &m);
    println!("n = {}, m = {}", n, m);
}

// 関数ポインタ
fn app_n(f: fn(u64) -> u64, mut n: u64, mut x: u64) -> u64 {
   loop{
       if n == 0 {
        return x;
       }
       x = f(x);
       n -= 1;
   }
}

fn mul2(x: u64) -> u64 {
    x * 2
}

fn my_func3() {
    println!("app_n(mul2, 4, 3) = {}", app_n(mul2, 4, 3));
}