#[macro_use]
extern crate log;
use std::fmt::Formatter;
use std::rc::Rc;
use std::sync::Arc;
use std::{fmt, thread};

#[derive(Debug)]
struct Structure {
    Name: String,
    Age: u32,
}

fn main() {
    // Rc
    // Rc 用于同一线程内部，通过 use std::rc::Rc 来引入。它有以下几个特点：
    //
    // 用 Rc 包装起来的类型对象，是 immutable 的，即 不可变的。即你无法修改 Rc<T> 中的 T 对象，只能读；
    // 一旦最后一个拥有者消失，则资源会被自动回收，这个生命周期是在编译期就确定下来的；
    // Rc 只能用于同一线程内部，不能用于线程之间的对象共享（不能跨线程传递）；
    // Rc 实际上是一个指针，它不影响包裹对象的方法调用形式（即不存在先解开包裹再调用值这一说）。

    let five: Rc<Structure> = Rc::new(Structure {
        Name: String::from("name01"),
        Age: 19,
    });
    // Rc Weak
    // 可访问，但不拥有。不增加引用计数，因此，不会对资源回收管理造成影响；
    // 可由 Rc<T> 调用 downgrade 方法而转换成 Weak<T>；
    // Weak<T> 可以使用 upgrade 方法转换成 Option<Rc<T>>，如果资源已经被释放，则 Option 值为 None；
    // 常用于解决循环引用的问题

    let weak_five = Rc::downgrade(&five);
    let strong_five: Option<Rc<_>> = weak_five.upgrade();
    println!("{:?}", five);

    // Arc
    // Arc 是原子引用计数，是 Rc 的多线程版本。Arc 通过 std::sync::Arc 引入。
    //
    // 它的特点：
    //
    // Arc 可跨线程传递，用于跨线程共享一个对象；
    // 用 Arc 包裹起来的类型对象，对可变性没有要求；
    // 一旦最后一个拥有者消失，则资源会被自动回收，这个生命周期是在编译期就确定下来的；
    // Arc 实际上是一个指针，它不影响包裹对象的方法调用形式（即不存在先解开包裹再调用值这一说）；
    // Arc 对于多线程的共享状态几乎是必须的（减少复制，提高性能）。
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers = Arc::new(numbers);

    for _ in 0..10 {
        let child_numbers = shared_numbers.clone();
        thread::spawn(move || {
            let local_numbers = &child_numbers[..];

            // Work with the local numbers
        });
    }
}
