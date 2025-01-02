use futures::{executor::block_on, Future};
use std::pin::Pin;
use std::task::{Context, Poll};

fn main() {
    let f = get_point();
    block_on(f);
}

async fn get_point() {
    let p = Point { x: 1 };
    let result = p.await;
    println!("{}", result);
}

struct Point {
    x: i32,
}

impl Future for Point {
    type Output = i32;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.x == 10 {
            println!("start");
            Poll::Ready(10)
        } else {
            println!("Pending");
            Poll::Pending
        }
    }
}
