use futures::future::{BoxFuture, FutureExt};
use futures::task::{waker_ref, ArcWake};
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

struct Hello {
    // <1>
    state: StateHello,
}

// 状態 <2>
enum StateHello {
    HELLO,
    WORLD,
    END,
}

impl Hello {
    fn new() -> Self {
        Hello {
            state: StateHello::HELLO, // 初期状態
        }
    }
}

impl Future for Hello {
    type Output = i32;

    // 実行関数 <3>
    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        match (*self).state {
            StateHello::HELLO => {
                print!("Hello, ");
                // WORLD状態に遷移
                (*self).state = StateHello::WORLD;
                Poll::Pending // 再度呼び出し可能
            }
            StateHello::WORLD => {
                println!("World!");
                // END状態に遷移
                (*self).state = StateHello::END;
                Poll::Pending // 再度呼び出し可能
            }
            StateHello::END => {
                Poll::Ready(100) // 終了
            }
        }
    }
}

// 実行単位 <1>
struct Task {
    hello: Mutex<BoxFuture<'static, i32>>,
}

impl Task {
    fn new() -> Self {
        let hello = Hello::new();
        Task {
            hello: Mutex::new(hello.boxed()),
        }
    }
}

// 何もしない
impl ArcWake for Task {
    fn wake_by_ref(_arc_self: &Arc<Self>) {}
}

fn main() {
    // 初期化
    let task = Arc::new(Task::new());
    let waker = waker_ref(&task);
    let mut ctx = Context::from_waker(&waker); // <2>
    let mut hello = task.hello.lock().unwrap();

    // 停止と再開の繰り返し <3>
    let a = hello.as_mut().poll(&mut ctx);
    print!("1, {}\n", a.is_ready());

    let a = hello.as_mut().poll(&mut ctx);
    print!("2, {}\n", a.is_ready());
    let num = match a {
        Poll::Ready(t) => Some(t),
        Poll::Pending => None,
    };
    print!("result2, {:?}\n", num);

    let a = hello.as_mut().poll(&mut ctx);
    print!("3, {}\n", a.is_ready());
    let num = match a {
        Poll::Ready(t) => Some(t),
        Poll::Pending => None,
    };
    print!("result3, {:?}\n", num);
}
