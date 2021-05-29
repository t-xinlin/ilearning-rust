// #[macro_use]
// extern crate log;
// extern crate log4rs;
use std::error::Error;
use chrono::Utc;
use cron::Schedule;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{env, fs, thread};
use crate::cron::scheduler::Scheduler;
use futures::future::ok;

pub fn init() ->Result<(), Box<dyn Error>> {
    let mut cwd = env::current_dir().unwrap();
    cwd.push(Path::new("conf"));
    cwd.push(Path::new("log4rs.yaml"));
    println!("============={}", cwd.to_str().unwrap());
    let confPath = cwd.to_str().unwrap();

    // let paths = fs::read_dir("./").unwrap();
    // for path in paths {
    //     println!("Name: {}", path.unwrap().path().display())
    // }
    // return;

    log4rs::init_file(confPath, Default::default()).unwrap();
    //               sec  min   hour   day of month   month   day of week   year
    // let expression: &str = "0 * * * * *";
    // let schedule: Schedule = Schedule::from_str(expression).unwrap();
    // thread::spawn(move || {
    //     for datetime in schedule.upcoming(Utc) {
    //         info!("All stars: -> {}", datetime);
    //     }
    // });

    // for datetime in schedule.upcoming(Utc) {
    //     debug!("All stars: -> {}", datetime);
    // }
    Ok(())
}
