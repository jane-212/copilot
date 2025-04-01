pub mod normal;

use std::fmt::Display;

use anyhow::Result;
use futures::future::BoxFuture;

pub trait Task: Send + Sync + 'static + Display {
    fn job(&self) -> &'static str;
    fn run(&self) -> BoxFuture<Result<()>>;
}
