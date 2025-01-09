use std::{future::Future, pin::Pin, sync::Arc};

use anyhow::{bail, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tera::Context;

use super::Task;
use crate::helper::Helper;

pub struct Normal {
    helper: Arc<Helper>,
    client: Client,
}

impl Normal {
    pub fn new(helper: Arc<Helper>) -> Self {
        let client = Client::new();

        Self { client, helper }
    }

    async fn daily(&self, mut context: Context) -> Result<Context> {
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

        context.insert("daily", &response.data);

        Ok(context)
    }

    async fn it(&self, mut context: Context) -> Result<Context> {
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

        context.insert("it", &response.data);

        Ok(context)
    }
}

impl Task for Normal {
    fn job(&self) -> &'static str {
        "0 0 6 * * *"
    }

    fn description(&self) -> &'static str {
        "发送每日一句和IT资讯"
    }

    fn run(&self) -> Pin<Box<dyn Future<Output = Result<()>> + '_ + Send>> {
        Box::pin(async move {
            let context = Context::new();
            let context = self.daily(context).await?;
            let context = self.it(context).await?;
            let html = self.helper.tera.render("normal.html", &context)?;

            self.helper.mailer.send("Normal", html).await?;

            Ok(())
        })
    }
}
