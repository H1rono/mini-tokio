use std::future::Future;
use std::sync::{mpsc, Arc};

mod delay;
mod notify;
mod task;

pub use delay::Delay;
pub use notify::Notify;
pub use task::Task;

pub struct MiniTokio {
    scheduled: mpsc::Receiver<Arc<Task>>,
    sender: mpsc::Sender<Arc<Task>>,
}

impl MiniTokio {
    /// Initialize a new mini-tokio instance.
    pub fn new() -> Self {
        let (sender, scheduled) = mpsc::channel();
        Self { scheduled, sender }
    }

    /// Spawn a future onto the mini-tokio instance.
    ///
    /// The given future is wrapped with the `Task` harness and pushed into the
    /// `scheduled` queue. The future will be executed when `run` is called.
    pub fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        Task::spawn(future, &self.sender);
    }

    pub fn run(&mut self) {
        while let Ok(task) = self.scheduled.recv() {
            task.poll();
        }
    }
}

impl Default for MiniTokio {
    fn default() -> Self {
        Self::new()
    }
}
