use anyhow::Result;
use copilot::App;
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;
    env_logger::init();

    let app = App::new()?;

    app.start().await
}
