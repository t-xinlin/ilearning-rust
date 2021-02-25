use std::rc::Rc;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;

const N: usize = 10;
fn main1() {
    let data = Arc::new(Mutex::new(10));

    let (tx, rx) = channel();
    for _ in 0..10 {
        let (data, tx) = (data.clone(), tx.clone());
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            println!("run");
            *data += 1;
            if *data == N {
                tx.send(()).unwrap();
            }
        });
    }

    rx.recv().unwrap();
    println!("finish");
}

fn main() {
    // mux();
    lock();
}

// Mutex
// Mutex 意为互斥对象，用来保护共享数据。Mutex 有下面几个特征：
//
// Mutex 会等待获取锁令牌(token)，在等待过程中，会阻塞线程。直到锁令牌得到。同时只有一个线程的 Mutex 对象获取到锁；
// Mutex 通过 .lock() 或 .try_lock() 来尝试得到锁令牌，被保护的对象，必须通过这两个方法返回的 RAII 守卫来调用，不能直接操作；
// 当 RAII 守卫作用域结束后，锁会自动解开；
// 在多线程中，Mutex 一般和 Arc 配合使用。
// 示例：

fn mux() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1; //deref MutexGuard
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap()); //deref MutexGuard
}
// RwLock 翻译成 读写锁。它的特点是：
//
// 同时允许多个读，最多只能有一个写；
// 读和写不能同时存在；
//
// 读写锁的方法
// .read()
// .try_read()
// .write()
// .try_write()
// 注意需要对 .try_read() 和 .try_write() 的返回值进行判断。

fn lock() {
    use std::sync::RwLock;

    let lock = RwLock::new(5);

    // many reader locks can be held at once
    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        assert_eq!(*r1, 5);
        assert_eq!(*r2, 5);
    } // read locks are dropped at this point

    // only one write lock may be held, however
    {
        let mut w = lock.write().unwrap();
        *w += 1;
        assert_eq!(*w, 6);
    } // write lock is dropped here
}
