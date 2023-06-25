// #[macro_use]
// extern crate log;
// extern crate log4rs;
use std::error::Error;


use std::path::{Path};
// use std::str::FromStr;
use std::{env/*,  thread*/};
// use futures::future::ok;

pub fn init() ->Result<(), Box<dyn Error>> {
    let mut cwd = env::current_dir().unwrap();
    cwd.push(Path::new("conf"));
    cwd.push(Path::new("log4rs.yaml"));
    let conf_path = cwd.to_str().unwrap();
    log4rs::init_file(conf_path, Default::default()).unwrap();
    Ok(())
}
