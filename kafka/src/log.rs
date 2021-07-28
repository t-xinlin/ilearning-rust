// #[macro_use]

use std::path::Path;
use std::env;

pub fn log_init() /* ->Result<(), Box<dyn Error>> */
{
    let mut cwd = env::current_dir().unwrap();
    cwd.push(Path::new("conf"));
    cwd.push(Path::new("log4rs.yaml"));
    println!("init log path:{}", cwd.to_str().unwrap());
    let conf_path = cwd.to_str().unwrap();
    log4rs::init_file(conf_path, Default::default()).unwrap();
    // Ok(())
}
