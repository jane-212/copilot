use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;

use crate::app::Helper;

use super::app::Task;

pub(crate) struct Normal;

#[async_trait]
impl Task for Normal {
    fn description(&self) -> &'static str {
        "do some normal things"
    }

    async fn run(&self, helper: Arc<Helper>) -> Result<()> {
        let reply_from_deep_seek = helper.deep_seek.chat("hello").await?;
        let reply_from_openai = helper.openai.chat("hello").await?;
        helper
            .mailer
            .send(
                "normal test",
                format!(
                    "<p>deep seek: {reply_from_deep_seek}</p><p>openai: {reply_from_openai}</p>"
                ),
            )
            .await?;

        Ok(())
    }
}
