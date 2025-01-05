use anyhow::Result;
use copilot::App;
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    App::new()?.start().await
}
