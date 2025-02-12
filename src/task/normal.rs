use std::sync::Arc;

use anyhow::{bail, Result};
use askama::Template;
use futures::future::BoxFuture;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::Task;
use crate::helper::Helper;
use crate::templates;

pub struct Normal {
    helper: Arc<Helper>,
    client: Client,
}

impl Normal {
    pub fn new(helper: Arc<Helper>) -> Self {
        let client = Client::new();

        Self { client, helper }
    }

    async fn daily(&self) -> Result<(String, String)> {
        #[derive(Deserialize)]
        struct Response {
            success: bool,
            data: Data,
        }

        #[derive(Deserialize, Serialize)]
        struct Data {
            zh: String,
            en: String,
            pic: String,
        }

        let url = "https://api.vvhan.com/api/dailyEnglish";
        let response = self
            .client
            .get(url)
            .send()
            .await?
            .json::<Response>()
            .await?;
        if !response.success {
            bail!("接口请求失败: {url}");
        }

        Ok((response.data.zh, response.data.en))
    }

    async fn it(&self) -> Result<Vec<templates::New>> {
        #[allow(unused)]
        #[derive(Deserialize)]
        struct Response {
            success: bool,
            name: String,
            subtitle: String,
            update_time: String,
            data: Vec<New>,
        }

        #[derive(Deserialize, Serialize)]
        struct New {
            title: String,
            hot: String,
            url: String,
            mobil_url: String,
            index: usize,
        }

        let url = "https://api.vvhan.com/api/hotlist/itNews";
        let response = self
            .client
            .get(url)
            .send()
            .await?
            .json::<Response>()
            .await?;
        if !response.success {
            bail!("接口请求失败: {url}");
        }

        Ok(response
            .data
            .into_iter()
            .map(|new| {
                templates::New::builder()
                    .index(new.index)
                    .title(new.title)
                    .build()
            })
            .collect())
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

impl Task for Normal {
    fn job(&self) -> &'static str {
        "0 0 6 * * *"
    }

    fn description(&self) -> &'static str {
        "发送日常资讯"
    }

    fn run(&self) -> BoxFuture<Result<()>> {
        Box::pin(async move {
            let (zh, en) = self.daily().await?;
            let news = self.it().await?;
            let (star, download) = self.javcap().await?;
            let html = templates::Normal::builder()
                .zh(&zh)
                .en(&en)
                .star(star)
                .download(download)
                .news(&news)
                .build()
                .render()?;

            self.helper.mailer.send("Normal", html).await?;

            Ok(())
        })
    }
}
