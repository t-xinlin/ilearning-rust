use std::env;
use std::path::Path;

fn main() {
    let mut cwd = env::current_dir().unwrap();
    let confPath = Path::new("conf").join(Path::new("log4rs.yaml"));
    cwd = cwd.join(confPath);
    let confPath = cwd.to_str().unwrap();
    println!("============={}", confPath);
}
