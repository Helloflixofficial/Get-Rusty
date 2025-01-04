use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use futures::task::{waker_ref, ArcWake};

fn main() {
    let f = hello();
    block_on(Box::pin(f));
}

async fn hello() {
    println!("hello sire");
}

struct MyWaker;

impl ArcWake for MyWaker {
    fn wake_by_ref(_arc_self: &Arc<Self>) {
        println!("Waker invoked!");
    }
}

fn block_on(future: Pin<Box<dyn Future<Output = ()>>>) {
    let my_waker = Arc::new(MyWaker {});
    let waker = waker_ref(&my_waker);
    let mut context = Context::from_waker(&*waker);

    let mut future = future;
    match future.as_mut().poll(&mut context) {
        Poll::Ready(()) => println!("Ready"),
        Poll::Pending => println!("Pending"),
    }
}
