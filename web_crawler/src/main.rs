use web_crawler::run;

#[tokio::main]
async fn main() {
    run("https://www.willvelida.com/".to_string(), 10).await;
}
