pub mod normal;

use anyhow::Result;
use futures::future::BoxFuture;

pub trait Task: Send + Sync + 'static {
    fn job(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn run(&self) -> BoxFuture<Result<()>>;
}
