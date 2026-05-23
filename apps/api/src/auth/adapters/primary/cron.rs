use std::{sync::Arc, time::Duration};

use tracing::error;

use crate::{
    auth::ports::Handler,
    pkg::cron::{Scheduler, SchedulerExt},
};

pub struct CronAdapter {
    manager: Arc<dyn Scheduler>,
    core: Arc<dyn Handler>,
}

impl CronAdapter {
    pub fn new(manager: Arc<dyn Scheduler>, core: Arc<dyn Handler>) -> Self {
        let instance = Self { manager, core };
        instance.purge_refresh_tokens();
        instance
    }

    pub fn purge_refresh_tokens(&self) {
        let core = self.core.clone();
        self.manager.add(Duration::from_secs(10), move || {
            let core = core.clone();
            async move {
                let result = core.purge_refresh_tokens().await;
                if let Err(result) = result {
                    error!("Failed to purge refresh tokens: {}", result);
                }
            }
        });
    }
}
