use std::sync::Arc;

use anyhow::Result;
use copilot::task::normal::Normal;
use copilot::App;
use dotenvy::dotenv;
use self_update::backends::github::Update;
use self_update::Status;
use time::macros::format_description;
use time::UtcOffset;
use tracing_subscriber::fmt::time::OffsetTime;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let local_time = OffsetTime::new(
        UtcOffset::from_hms(8, 0, 0)?,
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second]"),
    );
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_timer(local_time)
        .init();

    let status = tokio::task::spawn_blocking(check_for_update).await??;
    if status.updated() {
        return Ok(());
    }

    let mut app = App::new()?;
    app.add_task(|helper| {
        let normal = Normal::new(helper)?;
        let task = Arc::new(normal);
        Ok(task)
    })?;

    app.start().await
}

fn check_for_update() -> Result<Status> {
    let status = Update::configure()
        .repo_owner("jane-212")
        .repo_name("copilot")
        .bin_name("copilot")
        .no_confirm(true)
        .show_output(false)
        .show_download_progress(false)
        .current_version(env!("VERSION"))
        .build()
        .with_context(|| "build update config")?
        .update()
        .with_context(|| "self update")?;

    Ok(status)
}
