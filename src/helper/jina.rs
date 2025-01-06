use anyhow::Result;
use reqwest::Client;

pub struct Jina {
    client: Client,
}

impl Jina {
    pub fn new() -> Self {
        let client = Client::new();

        Self { client }
    }

    pub async fn summary(
        &self,
        url: impl AsRef<str>,
        includes: impl Into<Vec<&str>>,
        excludes: impl Into<Vec<&str>>,
    ) -> Result<String> {
        let url = url.as_ref();
        let url = format!("https://r.jina.ai/{}", url);
        let summary = self
            .client
            .get(url)
            .header("X-Target-Selector", includes.into().join(", "))
            .header("X-Remove-Selector", excludes.into().join(", "))
            .send()
            .await?
            .text()
            .await?;

        Ok(summary)
    }
}
