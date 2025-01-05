use std::env;
use std::sync::Arc;

use anyhow::{Context, Result};
use async_trait::async_trait;
use tokio::signal::unix::{signal, SignalKind};
use tokio_cron_scheduler::{Job, JobScheduler};

use super::mail::Mailer;
use super::normal::Normal;
use super::openai::Openai;

pub struct App {
    helper: Arc<Helper>,
}

impl App {
    pub fn new() -> Result<Self> {
        let helper = Helper::new()?;
        let app = Self {
            helper: Arc::new(helper),
        };

        Ok(app)
    }

    pub async fn start(self) -> Result<()> {
        let mut sched = JobScheduler::new().await?;
        sched
            .add(Job::new_async("0 0 6,18 * * *", {
                let helper = self.helper.clone();
                move |_, _| {
                    let helper = helper.clone();
                    Box::pin(async move {
                        let normal = Normal;
                        normal
                            .run(helper.clone())
                            .await
                            .context(format!("failed when {}", normal.description()))
                            .log_err();
                    })
                }
            })?)
            .await?;
        sched.start().await?;

        let mut terminate = signal(SignalKind::terminate())?;
        tokio::select! {
            _ = terminate.recv() => (),
        }

        sched.shutdown().await?;

        Ok(())
    }
}

trait LogErr {
    fn log_err(&self);
}

impl LogErr for Result<()> {
    fn log_err(&self) {
        if let Err(error) = self {
            eprintln!("{error:?}");
        }
    }
}

#[async_trait]
pub trait Task: Send + Sync {
    fn description(&self) -> &'static str;
    async fn run(&self, helper: Arc<Helper>) -> Result<()>;
}

pub struct Helper {
    pub(crate) mailer: Mailer,
    pub(crate) openai: Openai,
    pub(crate) deep_seek: Openai,
}

impl Helper {
    fn new() -> Result<Self> {
        let mailer = Self::new_mailer()?;
        let openai = Self::new_openai()?;
        let deep_seek = Self::new_deep_seek()?;
        let helper = Self {
            mailer,
            openai,
            deep_seek,
        };

        Ok(helper)
    }

    fn new_deep_seek() -> Result<Openai> {
        let deep_seek_key = env::var("DEEP_SEEK_KEY")?;
        let deep_seek = Openai::new("https://api.deepseek.com", "deepseek-chat", deep_seek_key)?;

        Ok(deep_seek)
    }

    fn new_openai() -> Result<Openai> {
        let openai_key = env::var("OPENAI_KEY")?;
        let openai = Openai::new(
            "https://models.inference.ai.azure.com",
            "gpt-4o",
            openai_key,
        )?;

        Ok(openai)
    }

    fn new_mailer() -> Result<Mailer> {
        let to = env::var("MAIL_TO")?;
        let username = env::var("MAIL_USERNAME")?;
        let password = env::var("MAIL_PASSWORD")?;
        let mailer = Mailer::new(
            format!("Bot <{username}>"),
            to,
            "smtp.163.com",
            username,
            password,
        )?;

        Ok(mailer)
    }
}
