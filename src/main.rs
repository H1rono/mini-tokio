use std::time::Duration;

use mini_tokio::{Delay, MiniTokio};

fn main() {
    let mut mini_tokio = MiniTokio::new();
    mini_tokio.spawn(async {
        let delay = Delay::after(Duration::from_secs(1));
        let out = delay.await;
        println!("delay output: {}", out);
    });
    mini_tokio.run();
}
