#[macro_use]
extern crate log;
extern crate log4rs;

use chrono::Utc;
use cron::Schedule;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{env, fs, thread};

fn main() {
    let mut cwd = env::current_dir().unwrap();
    let p = Path::new("log4rs.yaml");
    cwd.push(p);
    println!("============={}", cwd.to_str().unwrap());
    let confPath = cwd.to_str().unwrap();

    // let paths = fs::read_dir("./").unwrap();
    // for path in paths {
    //     println!("Name: {}", path.unwrap().path().display())
    // }
    // return;

    log4rs::init_file(confPath, Default::default()).unwrap();

    let expression: &str = "* 1 * * * * *";
    let schedule: Schedule = Schedule::from_str(expression).unwrap();

    // thread::spawn(move || {
    //     for datetime in schedule.upcoming(Utc) {
    //         info!("All stars: -> {}", datetime);
    //     }
    // });

    for datetime in schedule.upcoming(Utc) {
        debug!("All stars: -> {}", datetime);
    }
}
