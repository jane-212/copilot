use std::sync::Arc;

use anyhow::{Context, Result};
use tokio::signal::unix::{signal, SignalKind};
use tokio_cron_scheduler::{JobBuilder, JobScheduler};

use super::helper::Helper;
use super::task::Task;

pub struct App {
    helper: Arc<Helper>,
    tasks: Vec<Arc<dyn Task>>,
}

impl App {
    pub fn new() -> Result<Self> {
        let helper = Helper::new()?;
        let app = Self {
            helper: Arc::new(helper),
            tasks: Vec::new(),
        };

        Ok(app)
    }

    pub fn add_task(&mut self, task: Arc<dyn Task>) {
        self.tasks.push(task);
    }

    fn all_tasks(&self) -> Vec<Arc<dyn Task>> {
        self.tasks.clone()
    }

    pub async fn start(self) -> Result<()> {
        log::info!("start app...");
        log::info!("app version: {}", env!("VERSION"));
        let mut sched = JobScheduler::new().await?;
        let tasks = self.all_tasks();
        log::info!("totally found {} tasks", tasks.len());
        for task in tasks {
            log::info!("add new job: {}({})", task.description(), task.job());
            let job = JobBuilder::new()
                .with_timezone(chrono_tz::Asia::Shanghai)
                .with_cron_job_type()
                .with_schedule(task.job())?
                .with_run_async(Box::new({
                    let helper = self.helper.clone();
                    move |_, _| {
                        let task = task.clone();
                        let helper = helper.clone();
                        Box::pin(async move {
                            log::info!("start {}", task.description());
                            match task
                                .run(helper.clone())
                                .await
                                .context(format!("failed when {}", task.description()))
                            {
                                Ok(_) => log::info!("finished {}", task.description()),
                                Err(error) => log::error!("{:?}", error),
                            }
                        })
                    }
                }))
                .build()?;
            sched.add(job).await?;
        }
        log::info!("app started and wait for quit signal...");
        sched.start().await?;

        let mut terminate = signal(SignalKind::terminate())?;
        tokio::select! {
            _ = terminate.recv() => (),
        }
        log::info!("received terminate signal");

        log::info!("shutdown app...");
        sched.shutdown().await?;

        log::info!("shutdown app successfully");
        Ok(())
    }
}
