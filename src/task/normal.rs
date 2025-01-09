use std::{future::Future, pin::Pin, sync::Arc};

use anyhow::Result;
use tera::Context;

use super::{Helper, Task};

pub struct Normal;

impl Task for Normal {
    #[cfg(debug_assertions)]
    fn job(&self) -> &'static str {
        "*/5 * * * * *"
    }
    
    #[cfg(not(debug_assertions))]
    fn job(&self) -> &'static str {
        "0 0 6 * * *"
    }

    fn description(&self) -> &'static str {
        "发送日历"
    }

    fn run(&self, helper: Arc<Helper>) -> Pin<Box<dyn Future<Output = Result<()>> + '_ + Send>> {
        Box::pin(async move {
            let mut context = Context::new();
            context.insert("date_image", "https://api.vvhan.com/api/moyu");
            let html = helper.tera.render("normal.html", &context)?;
            helper.mailer.send("Normal", html).await?;

            Ok(())
        })
    }
}
