pub mod normal;

use std::{future::Future, pin::Pin};

use anyhow::Result;

pub trait Task: Send + Sync + 'static {
    fn job(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn run(&self) -> Pin<Box<dyn Future<Output = Result<()>> + '_ + Send>>;
}
