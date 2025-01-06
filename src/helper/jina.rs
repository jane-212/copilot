use std::time::Duration;

use anyhow::Result;
use reqwest::Client;

pub struct Jina {
    client: Client,
}

impl Jina {
    pub fn new() -> Result<Self> {
        let client = Client::builder().timeout(Duration::from_secs(10)).build()?;
        let jina = Self { client };

        Ok(jina)
    }

    pub async fn summary(&self, url: impl AsRef<str>) -> Result<String> {
        let url = url.as_ref();
        let url = format!("https://r.jina.ai/{}", url);
        let summary = self.client.get(url).send().await?.text().await?;

        Ok(summary)
    }
}
