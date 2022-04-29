use rand::Rng;
use std::time::Duration;
use tokio::{select, sync::mpsc, time::sleep};

#[tokio::main]
async fn main() {
    let (tx1, mut rx1) = mpsc::unbounded_channel();
    let (tx2, mut rx2) = mpsc::unbounded_channel();
    tokio::spawn(async move {
        for i in 1..=10 {
            tx1.send(i).unwrap();
            let d = rand::thread_rng().gen_range(1..100_000);
            sleep(Duration::from_micros(d)).await;
        }
    });
    tokio::spawn(async move {
        for i in 1..=10 {
            tx2.send(i).unwrap();
            let d = rand::thread_rng().gen_range(1..100_000);
            sleep(Duration::from_micros(d)).await;
        }
    });
    loop {
        select! {
            Some(val) = rx1.recv() => {
                println!("task 1 sent {val}");
            }
            Some(val) = rx2.recv() => {
                println!("task 2 sent {val}");
            }
            else => {
                println!("All done");
                break;
            }
        }
    }
}
