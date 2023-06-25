use std::path::{Path};
use std::{env};
use log4rs;
use std::error::Error;

// /// function init log
// pub fn init_log() {
//     let mut cwd = env::current_dir().unwrap();
//     cwd.push(Path::new("conf"));
//     cwd.push(Path::new("log4rs.toml"));
//     let conf_path = cwd.to_str().unwrap();
//     println!("log config file: {}", conf_path);
//     log4rs::init_file(conf_path, Default::default()).unwrap();
// }

pub fn init() ->Result<(), Box<dyn Error>> {
    let mut cwd = env::current_dir().unwrap();
    cwd.push(Path::new("conf"));
    cwd.push(Path::new("log4rs.yaml"));
    let conf_path = cwd.to_str().unwrap();
    log4rs::init_file(conf_path, Default::default()).unwrap();
    Ok(())
}
