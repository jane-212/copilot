use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use tera::Context;

use super::Task;
use crate::helper::Helper;

pub struct Log;

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
        let log_html = helper.tera.render("log.html", &context)?;
        helper
            .mailer
            .send("Log summary for last 7 days", log_html)
            .await?;

        Ok(())
    }
}
