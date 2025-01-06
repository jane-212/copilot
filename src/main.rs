use std::sync::Arc;

use anyhow::Result;
use copilot::{task::log::Log, App};
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let mut app = App::new()?;
    app.add_task(Arc::new(Log));

    app.start().await
}
