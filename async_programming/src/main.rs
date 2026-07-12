use tokio::join;
use tokio_stream::{self as stream, StreamExt};

#[tokio::main]
async fn main() {
    print_numbers().await;
}

async fn fetch_data() -> String {
    String::from("data")
}

async fn run() {
    let data = fetch_data().await;
    println!("{data}");
}

async fn two_things() {
    let (a, b) = join!(fetch_data(), fetch_data());
    println!("{a} {b}");
}

async fn spawn_task() {
    let handle = tokio::spawn(async {
        fetch_data().await
    });
    let result = handle.await.unwrap();
    println!("{result}");
}

async fn print_numbers() {
    let mut stream = stream::iter(vec![1, 2, 3]);
    while let Some(n) = stream.next().await {
        println!("{n}");
    }
}