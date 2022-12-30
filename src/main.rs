use async_std::task;

mod server;

async fn c2_cli() {
    server::srv();
}

#[async_std::main]
async fn main() {
    task::spawn(c2_cli()).await;
}
