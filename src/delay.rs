use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread;
use std::time::{Duration, Instant};

/// An async object, which is represented as a state machine
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
            // Get a handle to the waker for the current task
            let waker = cx.waker().clone();
            let when = self.when;

            // Spawn a timer thread.
            thread::spawn(move || {
                // `wake`-up waker after this future get ready to make transition.
                let now = Instant::now();

                if now < when {
                    thread::sleep(when - now);
                }

                waker.wake();
            });
            Poll::Pending
        }
    }
}
