use std::time::Duration;

use tokio::select;
use tokio_util::sync::CancellationToken;

use crate::pkg::cron::{CronFuture, Scheduler as CronManagerImpl};

pub struct CronScheduler {
    signal: CancellationToken,
}

impl CronScheduler {
    pub fn new(signal: CancellationToken) -> Self {
        Self { signal }
    }
}

impl CronManagerImpl for CronScheduler {
    fn add_boxed(
        &self,
        duration: Duration,
        mut task: Box<dyn FnMut() -> CronFuture + Send + 'static>,
    ) {
        let signal = self.signal.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(duration);

            loop {
                select! {
                    _ = interval.tick() => {
                        task().await;
                    }

                    _ = signal.cancelled() => {
                        break;
                    }
                }
            }
        });
    }
}
