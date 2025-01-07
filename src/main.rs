use std::sync::Arc;

use anyhow::Result;
use copilot::task::github::Github;
use copilot::App;
use dotenvy::dotenv;
use time::macros::format_description;
use time::UtcOffset;
use tracing_subscriber::fmt::time::OffsetTime;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let local_time = OffsetTime::new(
        UtcOffset::from_hms(8, 0, 0)?,
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"),
    );
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_timer(local_time)
        .init();

    let mut app = App::new()?;
    app.add_task(Arc::new(Github));

    app.start().await
}
