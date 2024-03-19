use dotenvy::dotenv;
use server::app::App;

#[tokio::main]
async fn main() {
    dotenv().unwrap();
    App::default().serve().await.unwrap();
}
