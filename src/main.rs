#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv::dotenv().ok();
    updoot::run().await;
}
