use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use tera::Context;

use crate::app::Helper;

use super::app::Task;

pub(crate) struct Log;

#[async_trait]
impl Task for Log {
    fn job(&self) -> &'static str {
        "0 0 0 */7 * *"
    }

    fn description(&self) -> &'static str {
        "send logs to email"
    }

    async fn run(&self, helper: Arc<Helper>) -> Result<()> {
        let logs = {
            let mut logger = helper.logger.lock().await;
            logger.take_all()
        };
        let mut context = Context::new();
        context.insert("logs", &logs);
        context.insert("count", &7);
        let log_html = helper.tera.render("log.html", &context)?;
        helper.mailer.send("Log summary", log_html).await?;

        Ok(())
    }
}
