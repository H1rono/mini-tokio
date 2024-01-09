use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

pub struct Delay {
    when: Instant,
}

impl Delay {
    pub fn new(when: Instant) -> Self {
        Self { when }
    }

    pub fn after(duration: Duration) -> Self {
        let when = Instant::now() + duration;
        Self::new(when)
    }
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<&'static str> {
        if Instant::now() >= self.when {
            println!("Hello world");
            Poll::Ready("done")
        } else {
            // Ignore this line for now.
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
