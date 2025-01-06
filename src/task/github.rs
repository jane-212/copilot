use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use tera::Context;

use super::{Helper, Task};

pub struct Github;

#[async_trait]
impl Task for Github {
    fn job(&self) -> &'static str {
        "0 0 6 * * *"
    }

    fn description(&self) -> &'static str {
        "get github daily trending"
    }

    async fn run(&self, helper: Arc<Helper>) -> Result<()> {
        let url = "https://github.com/trending/rust?since=daily";
        let trending = helper
            .jina
            .summary(url, [".Box-row"], [".d-inline-block"])
            .await?;
        let summary = helper
            .openai
            .chat(format!(
                "请帮我总结一下所有的项目并按照顺序star的多少排列, {trending}"
            ))
            .await?;
        let mut context = Context::new();
        context.insert("trending", &markdown::to_html(&summary));
        let html = helper.tera.render("github.html", &context)?;
        helper.mailer.send("Github rust trending", html).await?;

        Ok(())
    }
}