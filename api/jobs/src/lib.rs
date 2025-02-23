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

pub type Jobs = Vec<Box<dyn Job + Sync + Send + 'static>>;

/// This function starts the jobs. You have to call this function in your bootstrap code.
pub async fn start_jobs(jobs: Jobs) -> Result<(), JobSchedulerError> {
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
