use std::{future::Future, pin::Pin, time::Duration};

pub type CronFuture = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

pub trait Scheduler: Sync + Send {
    fn add_boxed(&self, duration: Duration, task: Box<dyn FnMut() -> CronFuture + Send + 'static>);
}

pub trait SchedulerExt: Scheduler {
    fn add<F, Fut>(&self, duration: Duration, mut task: F)
    where
        F: FnMut() -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.add_boxed(duration, Box::new(move || Box::pin(task())));
    }
}

impl<T> SchedulerExt for T where T: Scheduler + ?Sized {}
