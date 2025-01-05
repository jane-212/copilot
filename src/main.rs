use anyhow::Result;
use copilot::App;

#[tokio::main]
async fn main() -> Result<()> {
    App::new()?.start().await
}
