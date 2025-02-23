use std::{future::Future, pin::Pin};

use tokio_cron_scheduler::{Job as JobBuilder, JobScheduler, JobSchedulerError};
use uuid::Uuid;

// declare the modules here

pub trait Job {
    fn schedule(&self) -> String;
    fn run(
        &self,
        job_scheduler_lock: JobScheduler,
        uuid: Uuid,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>>;
}

pub struct Jobs {
    pub jobs: Vec<Box<dyn Job + Sync + Send + 'static>>,
}

impl Jobs {
    /// Create a new instance of `Jobs` with the given jobs.
    pub fn new(jobs: Vec<Box<dyn Job + Sync + Send + 'static>>) -> Self {
        Self { jobs }
    }
}

pub async fn start_jobs(
    jobs: Vec<Box<dyn Job + Sync + Send + 'static>>,
) -> Result<(), JobSchedulerError> {
    let sched = JobScheduler::new().await?;

    for job in jobs {
        sched
            .add(JobBuilder::new_async(job.schedule(), move |uuid, lock| {
                job.run(lock, uuid)
            })?)
            .await?;
    }

    sched.start().await?;

    Ok(())
}
