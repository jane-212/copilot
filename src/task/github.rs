use std::{future::Future, pin::Pin, sync::Arc};

use anyhow::Result;
use tera::Context;

use super::{Helper, Task};

pub struct Github;

impl Task for Github {
    fn job(&self) -> &'static str {
        "0 0 6 * * *"
    }

    fn description(&self) -> &'static str {
        "get github daily trending"
    }

    fn run(&self, helper: Arc<Helper>) -> Pin<Box<dyn Future<Output = Result<()>> + '_ + Send>> {
        Box::pin(async move {
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
        })
    }
}
