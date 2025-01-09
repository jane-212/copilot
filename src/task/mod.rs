pub mod normal;

use std::{future::Future, pin::Pin, sync::Arc};

use anyhow::Result;

use super::helper::Helper;

pub trait Task: Send + Sync + 'static {
    fn job(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn run(&self, helper: Arc<Helper>) -> Pin<Box<dyn Future<Output = Result<()>> + '_ + Send>>;
}
