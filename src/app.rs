use std::env;
use std::sync::Arc;

use anyhow::{Context, Result};
use async_trait::async_trait;
use tera::Tera;
use tokio::signal::unix::{signal, SignalKind};
use tokio::sync::Mutex;
use tokio_cron_scheduler::{Job, JobScheduler};

use super::log::Log;
use super::logger::Logger;
use super::mail::Mailer;
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

    fn all_tasks() -> Vec<Arc<dyn Task>> {
        vec![Arc::new(Log)]
    }

    pub async fn start(self) -> Result<()> {
        let mut sched = JobScheduler::new().await?;
        let tasks = Self::all_tasks();
        for task in tasks {
            sched
                .add(Job::new_async(task.job(), {
                    let helper = self.helper.clone();
                    move |_, _| {
                        let task = task.clone();
                        let helper = helper.clone();
                        Box::pin(async move {
                            if let Err(error) = task
                                .run(helper.clone())
                                .await
                                .context(format!("failed when {}", task.description()))
                            {
                                let mut logger = helper.logger.lock().await;
                                logger.error(error.to_string());
                            }
                        })
                    }
                })?)
                .await?;
        }
        sched.start().await?;

        let mut terminate = signal(SignalKind::terminate())?;
        tokio::select! {
            _ = terminate.recv() => (),
        }

        sched.shutdown().await?;

        Ok(())
    }
}

#[async_trait]
pub trait Task: Send + Sync {
    fn job(&self) -> &'static str;
    fn description(&self) -> &'static str;
    async fn run(&self, helper: Arc<Helper>) -> Result<()>;
}

pub struct Helper {
    pub(crate) mailer: Mailer,
    pub(crate) openai: Openai,
    pub(crate) deep_seek: Openai,
    pub(crate) logger: Mutex<Logger>,
    pub(crate) tera: Tera,
}

impl Helper {
    fn new() -> Result<Self> {
        let mailer = Self::new_mailer()?;
        let openai = Self::new_openai()?;
        let deep_seek = Self::new_deep_seek()?;
        let logger = Logger::new();
        let tera = Self::new_tera()?;
        let helper = Self {
            mailer,
            openai,
            deep_seek,
            logger: Mutex::new(logger),
            tera,
        };

        Ok(helper)
    }

    fn new_tera() -> Result<Tera> {
        let mut tera = Tera::default();
        tera.add_raw_template("base.html", include_str!("../templates/base.html"))?;
        tera.add_raw_template("log.html", include_str!("../templates/log.html"))?;

        Ok(tera)
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
