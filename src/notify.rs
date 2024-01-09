use std::pin::Pin;
use std::sync::atomic::Ordering;
use std::sync::{atomic::AtomicBool, Arc, Mutex};
use std::task::{Context, Poll, Waker};

use futures::Future;

#[derive(Default)]
pub struct Notify {
    notified: AtomicBool,
    waker: Option<Arc<Mutex<Waker>>>,
}

impl Notify {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn notify(&self) {
        self.notified.store(true, Ordering::Relaxed);
        let Some(waker) = &self.waker else {
            return;
        };
        waker.lock().unwrap().wake_by_ref();
    }
}

impl Future for Notify {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.notified.load(Ordering::Acquire) {
            return Poll::Ready(());
        }
        if let Some(waker) = &self.waker {
            let mut waker = waker.lock().unwrap();
            if !waker.will_wake(cx.waker()) {
                *waker = cx.waker().clone();
            }
        } else {
            let waker = Arc::new(Mutex::new(cx.waker().clone()));
            self.waker = Some(waker);
        }
        Poll::Pending
    }
}
