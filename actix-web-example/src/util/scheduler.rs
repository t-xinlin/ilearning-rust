// use chrono::{TimeZone, Utc};
// use cron::Schedule;
// use futures_util::future::join_all;
// use std::error::Error;
// use std::future::Future;
// use std::pin::Pin;
//
// use std::str::FromStr;
// use tokio::time::sleep;
//
// pub async fn RunScheduler(){
//     Scheduler::new()
//         .set_context(())
//         .job("* * * * * *", &|_| { Box::pin(async { debug!("Every second!") }) })
//         .start().await;
// }
//
// /// Cronic Scheduler
// pub struct Scheduler<C: 'static + Sized + Default + Clone> {
//     context: Option<C>,
//     jobs: Vec<(String, &'static dyn Fn(C) -> Pin<Box<dyn Future<Output=()>>>)>,
// }
//
// impl<'a, C: 'static + Default + Clone> Scheduler<C> {
//     pub fn new() -> Self {
//         Self {
//             context: None,
//             jobs: Vec::new(),
//         }
//     }
//
//     pub fn job<S: Into<String>>(mut self, cron_str: S, job: &'static impl Fn(C) -> Pin<Box<dyn Future<Output=()>>>) -> Self
//     {
//         self.jobs.push((cron_str.into(), job));
//         self
//     }
//
//     pub fn set_context(mut self, context: C) -> Self {
//         self.context = Some(context);
//         self
//     }
//
//     /// Start the execution of the scheduled jobs
//     pub async fn start(self) -> Result<(), Box<dyn Error>> {
//         // Set up schedules
//
//         let context = self.context.unwrap_or_default();
//
//         let mut futures = Vec::new();
//
//         for (cron, job) in self.jobs {
//             let con = context.clone();
//             let schedule = Schedule::from_str(&cron)?;
//
//             let fut = async move {
//                 for datetime in schedule.upcoming(Utc) {
//                     let now = Utc::now();
//
//                     if let Ok(duration) = datetime.signed_duration_since(now).to_std() {
//                         sleep(duration).await;
//                         let _result = job(con.clone()).await;
//                     }
//                 }
//             };
//
//             futures.push(fut);
//         }
//
//         join_all(futures).await;
//
//         Ok(())
//     }
// }
use tokio_cron_scheduler::{
    Job,
    JobScheduler,
};
use tokio::time::Duration;

pub async fn run_scheduler() {
    let mut sched = JobScheduler::new();

    sched.add(Job::new("1/1 * * * * *", |_uuid, _l| {
        info!("I run every 1 seconds");
    }).unwrap()).unwrap();

    sched.add(Job::new("1/30 * * * * *", |_uuid, _l| {
        info!("I run every 30 seconds");
    }).unwrap()).unwrap();

    sched.add(
        Job::new_one_shot(Duration::from_secs(18), |_uuid, _l| {
            info!("{:?} I'm only run once", chrono::Utc::now());
        }).unwrap()
    ).unwrap();

    let jj = Job::new_repeated(Duration::from_secs(8), |_uuid, _l| {
        info!("{:?} I'm repeated every 8 seconds", chrono::Utc::now());
    }).unwrap();
    sched.add(jj).unwrap();

    sched.start().await.unwrap();
}
