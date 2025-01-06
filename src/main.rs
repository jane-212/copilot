use anyhow::Result;
use copilot::App;
use dotenvy::dotenv;
use env_logger::Builder;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;
    Builder::from_default_env().format_target(false).init();

    let app = App::new()?;

    app.start().await
}
