use std::sync::Arc;

use anyhow::Result;
use copilot::task::github::Github;
use copilot::App;
use dotenvy::dotenv;
use env_logger::Builder;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;
    Builder::from_default_env().format_target(false).init();

    let mut app = App::new()?;
    app.add_task(Arc::new(Github));

    app.start().await
}
