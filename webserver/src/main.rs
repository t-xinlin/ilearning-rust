mod pool;

use std::net::{TcpListener, TcpStream};
use std::io::Read;
use std::io::prelude::*;
use std::fs;

use self::pool::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // let stream = stream.unwrap();

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            hand(stream);
        });
    }
}

fn hand(mut stream: TcpStream) {
    let mut buff = [0; 1024];
    stream.read(&mut buff).unwrap();
    //print!("Reqquesr{}", String::from_utf8_lossy(&buff));

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}