use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;

use super::helper::Helper;

#[async_trait]
pub trait Task: Send + Sync {
    fn job(&self) -> &'static str;
    fn description(&self) -> &'static str;
    async fn run(&self, helper: Arc<Helper>) -> Result<()>;
}
