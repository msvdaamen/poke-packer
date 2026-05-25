use std::time::Duration;

use shared::cron::{CronFuture, Scheduler as CronManagerImpl};

pub struct CronScheduler;

impl CronScheduler {
    pub fn new() -> Self {
        Self {}
    }
}

impl CronManagerImpl for CronScheduler {
    fn add_boxed(
        &self,
        duration: Duration,
        mut task: Box<dyn FnMut() -> CronFuture + Send + 'static>,
    ) {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(duration);

            loop {
                interval.tick().await;
                task().await;
            }
        });
    }
}
