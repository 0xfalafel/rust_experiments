use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<&'static str> {
        
        if Instant::now() >= self.when {
            println!("Legendary!");
            Poll::Ready("done")
        } else {
            // Ignore this line for now
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

#[tokio::main]
async fn main() {
    println!("Wait for it!");
    println!("It's gonna be ...");

    let when = Instant::now() + Duration::from_millis(2000);
    let future = Delay { when };

    let out = future.await;
    assert_eq!(out, "done");
}
