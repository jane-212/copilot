use std::fmt::{self, Display};
use std::sync::Arc;

use anyhow::Result;
use askama::Template;
use futures::future::BoxFuture;

use super::Task;
use crate::helper::Helper;
use crate::templates;

pub struct Normal {
    helper: Arc<Helper>,
}

impl Normal {
    pub fn new(helper: Arc<Helper>) -> Result<Self> {
        Ok(Self { helper })
    }

    async fn javcap(&self) -> Result<(u32, i64)> {
        let star_cnt = self
            .helper
            .github
            .repos("jane-212", "javcap")
            .get()
            .await?
            .stargazers_count
            .unwrap_or_default();
        let download_cnt = self
            .helper
            .github
            .repos("jane-212", "javcap")
            .releases()
            .list()
            .send()
            .await?
            .into_iter()
            .flat_map(|release| release.assets)
            .map(|asset| asset.download_count)
            .sum::<i64>();

        Ok((star_cnt, download_cnt))
    }
}

impl Display for Normal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "发送日常资讯")
    }
}

impl Task for Normal {
    fn job(&self) -> &'static str {
        "0 0 6 * * *"
    }

    fn run(&self) -> BoxFuture<Result<()>> {
        Box::pin(async move {
            let (star, download) = self.javcap().await?;
            let html = templates::Normal::builder()
                .star(star)
                .download(download)
                .build()
                .render()?;

            self.helper.mailer.send("Normal", html).await?;

            Ok(())
        })
    }
}
