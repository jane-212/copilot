use std::time::Duration;

use anyhow::Result;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::builder().timeout(Duration::from_secs(10)).build()?;
    let url = "https://rustcc.cn/article?id=f6804c11-951b-4bd1-b5b8-08bdf845750d";
    let text = client.get(url).send().await?.text().await?;
    println!("text: {text}");

    Ok(())
}
