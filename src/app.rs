use std::sync::Arc;

use anyhow::{Context, Error, Result};
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

    pub fn add_task(&mut self, handler: impl FnOnce(Arc<Helper>) -> Arc<dyn Task>) {
        self.tasks.push(handler(self.helper.clone()));
    }

    fn all_tasks(&self) -> Vec<Arc<dyn Task>> {
        self.tasks.clone()
    }

    async fn send_error(helper: Arc<Helper>, error: Error) -> Result<()> {
        log::info!("开始发送错误邮件");
        let error = format!("{error:?}");
        let error_lines = error.lines().collect::<Vec<_>>();

        let mut context = tera::Context::new();
        context.insert("error_lines", &error_lines);
        let error_html = helper.tera.render("error.html", &context)?;

        helper.mailer.send("检测到错误", error_html).await?;
        log::info!("错误邮件发送成功");

        Ok(())
    }

    pub async fn start(self) -> Result<()> {
        log::info!("正在启动...");
        log::info!("当前版本: {}", env!("VERSION"));

        let mut sched = JobScheduler::new().await?;
        let tasks = self.all_tasks();
        log::info!("一共发现{}个任务", tasks.len());
        for task in tasks {
            log::info!("正在添加任务: {}({})", task.description(), task.job());
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
                            log::info!("任务开始: {}", task.description());
                            match task
                                .run()
                                .await
                                .context(format!("任务失败: {}", task.description()))
                            {
                                Ok(_) => log::info!("任务完成: {}", task.description()),
                                Err(error) => {
                                    log::error!("\n{error:?}");
                                    if let Err(err) = Self::send_error(helper, error)
                                        .await
                                        .context("发送错误邮箱")
                                    {
                                        log::error!("\n{err:?}");
                                    }
                                }
                            }
                        })
                    }
                }))
                .build()?;
            sched.add(job).await?;
        }
        log::info!("启动成功, 正在监听停止信号...");
        sched.start().await?;

        let mut terminate = signal(SignalKind::terminate())?;
        tokio::select! {
            _ = terminate.recv() => (),
        }
        log::info!("收到停止信号");

        log::info!("正在退出...");
        sched.shutdown().await?;

        log::info!("退出成功");
        Ok(())
    }
}
