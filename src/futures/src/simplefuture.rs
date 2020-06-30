use std::thread;
use std::time::Duration;

enum Poll<T> {
    Ready(T),
    Pending,
}

trait SimpleFuture {
    type Output;
    //fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
    fn poll(&mut self, wake: u32) -> Poll<Self::Output>;
}

static mut FINISHED: bool = false;

struct MySleeper {
    polls: u64,
    wake: u32,
}

impl MySleeper {
    fn new() -> Self {
        MySleeper {
            polls: 0,
            wake: 0,
        }
    }
}

impl SimpleFuture for MySleeper {
    type Output = ();
    fn poll(&mut self, wake: u32) -> Poll<()> {
        unsafe {
            if FINISHED {
                Poll::Ready(())
            } else {
                self.wake = wake;
                self.polls += 1;
                println!("not ready yet --> {}", self.polls);
                Poll::Pending
            }
        }
    }
}
struct MyReactor {
    wake: u32,
    handle: Option<thread::JoinHandle<()>>,
}

impl MyReactor {
    fn new() -> MyReactor {
        MyReactor {
            wake: 0,
            handle: None,
        }
    }

    fn add_wake(&mut self, wake: u32) {
        self.wake = wake;
    }

    fn check_status(&mut self) {
        if self.handle.is_none() {
            let _wake = self.wake;
            let handle = thread::spawn(|| loop {
                thread::sleep(Duration::from_secs(5));
                {//模拟执行wake函数
                    unsafe {
                        FINISHED = true;
                    }
                }
            });

            self.handle = Some(handle);
        }
    }
}

struct MyExecutor;

impl MyExecutor {
    fn block_on<F: SimpleFuture>(mut my_future: F, wake: u32) {
        loop {
            match my_future.poll(wake) {
                Poll::Ready(_) => {
                    println!("my future execute ok!");
                    break;
                },
                Poll::Pending => {
                    unsafe {
                        while !FINISHED {//FINISHED为true表示为唤醒
                            thread::sleep(Duration::from_secs(1));
                        }
                    }
                }
            }
        }
    }
}

#[tokio::main]
fn main() {
    let mut reactor = MyReactor::new();
    let sleeper = MySleeper::new();
    let wake = sleeper.wake;
    reactor.add_wake(wake);
    reactor.check_status();
    MyExecutor::block_on(sleeper, wake);
}