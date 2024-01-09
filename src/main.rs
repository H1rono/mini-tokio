use std::time::Duration;

use mini_tokio::{Delay, MiniTokio};

fn main() {
    let mut mini_tokio = MiniTokio::new();
    for i in 1..=3 {
        mini_tokio.spawn(async move {
            let delay = Delay::after(Duration::from_secs(i));
            let out = delay.await;
            println!("delay output of {}: {}", i, out);
        });
    }
    mini_tokio.run();
}
