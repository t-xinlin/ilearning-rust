use std::time::{SystemTime, UNIX_EPOCH};

use async_trait::async_trait;
use tokio_cron_scheduler::{JobScheduler, Job, JobSchedulerError};
use log::*;

#[async_trait]
pub trait JobTrait {
    /// run job
    async fn run(&self, expression: &str) -> Result<(), JobSchedulerError>;
}

pub struct SchedulerJob;

pub fn build() -> Result<SchedulerJob, &'static str> {
    Ok(SchedulerJob {})
}

#[async_trait]
impl JobTrait for SchedulerJob {
    async fn run(&self, expression: &str) -> Result<(), JobSchedulerError> {
        // info!("async trait run")

        // let expression: &str = "1/1 * * * * *";
        let sched = JobScheduler::new().await?;

        // sched.shutdown_on_ctrl_c();
        // sched.set_shutdown_handler(Box::new(|| {
        //     Box::pin(async move {
        //         println!("Shut down done");
        //     })
        // }));

        let job_async = Job::new_async(expression, |_uuid, _l| {
            Box::pin(async move {
                match SystemTime::now().duration_since(UNIX_EPOCH) {
                    Ok(n) => info!("MicroSec：{:?}", n.as_micros()),
                    Err(_) => error!("System time error")
                }
            })
        })?;
        sched.add(job_async).await?;
        sched.start().await?;
        Ok(())
    }
}
