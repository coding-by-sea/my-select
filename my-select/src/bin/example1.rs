use std::time::Duration;
// run the asynchronous function until it completes or an even integer is received on the channel
use tokio;
use tokio::select;
use tokio::sync::mpsc;
use tokio::time::sleep;

async fn action() {
    sleep(Duration::from_millis(500)).await;
    println!("running action");
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<u8>(8);

    tokio::spawn(async move {
        tx.send(1).await.unwrap();
        tx.send(2).await.unwrap();
        tx.send(3).await.unwrap();
    });

    let operation = action();
    tokio::pin!(operation);

    loop {
        select! {
        _ = &mut operation => {
            println!("action completed");
        },
        Some(n) = rx.recv() => {
                println!("received: {:?}", n);
                if n % 2 == 0 {
                    break;
                }
        }
    }
    }

    println!("Hello, world!");
}
