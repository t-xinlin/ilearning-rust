use {
    std::{
        // future::Future,
        pin::Pin,
        sync::{Arc, Mutex},
        task::{Context, /*Poll,*/ Waker},
    },
};


fn main() {
    println!("Hello, world!");
}

trait SimpleFuture {
    type Output;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T),
    Pending,
}

struct SocketRead<'a> {
    socket: &'a Socket,
}

impl SimpleFuture for SocketRead<'_> {
    type Output = Vec<u8>;

    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if self.socket.has_data_to_read() {
            // 套接字拥有数据就读取数据到缓冲区并返回数据.
            Poll::Ready(self.socket.read_buf())
        } else {
            // 套接字没有数据
            // 安排 `wake` 在有数据之后能够被调用.
            // 当数据可获得的时候, `wake` 将被调用
            // 并且这个`Future` 的用户将知道再一次调用 `poll` 接收数据
            self.socket.set_readable_callback(wake);
            Poll::Pending
        }
    }
}


/// 一个SimpleFuture，可以同时运行另外两个future。
///
/// 并发是通过以下事实实现的：每个future都需要进行“poll”
/// 可能会交错，让每个future以自己的步伐前进。
pub struct Join<FutureA, FutureB> {
    // 每个字段都可能包含应运行以完成的future。
    // 如果future已经完成，则将该字段设置为“None”。
    // 这样可以防止我们在完成后轮询（poll）future。
    // 如果那样做就违反了`Future` 这个trait的契约（contract）.
    a: Option<FutureA>,
    b: Option<FutureB>,
}

impl<FutureA, FutureB> SimpleFuture for Join<FutureA, FutureB>
    where
        FutureA: SimpleFuture<Output=()>,
        FutureB: SimpleFuture<Output=()>,
{
    type Output = ();
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        // 尝试完成 future `a`.
        if let Some(a) = &mut self.a {
            if let Poll::Ready(()) = a.poll(wake) {
                self.a.take();
            }
        }

        // 尝试完成 future `b`.
        if let Some(b) = &mut self.b {
            if let Poll::Ready(()) = b.poll(wake) {
                self.b.take();
            }
        }

        if self.a.is_none() && self.b.is_none() {
            // 所有的futures都完成了，我们可以成功的返回
            Poll::Ready(())
        } else {
            // 一个或者全部的futures返回了`Poll::Pending`，说明仍有工作需要去做
            // 他们将会调用'wake()'，当取得进展时
            Poll::Pending
        }
    }
}

///  SimpleFuture 将一个接一个地运行知道完成
//
// 注意: 为了这个简单的例子, `AndThenFut` 假设两个future在创建时可用
// `AndThen` 组合器允许基于输出的第一个future创建第二个future
// 像这样使用： `get_breakfast.and_then(|food| eat(food))`.
pub struct AndThenFut<FutureA, FutureB> {
    first: Option<FutureA>,
    second: FutureB,
}

impl<FutureA, FutureB> SimpleFuture for AndThenFut<FutureA, FutureB>
    where
        FutureA: SimpleFuture<Output=()>,
        FutureB: SimpleFuture<Output=()>,
{
    type Output = ();
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if let Some(first) = &mut self.first {
            match first.poll(wake) {
                // 我们完成了第一个future，之后轮询它并完成第二个
                Poll::Ready(()) => self.first.take(),
                // 我们还不能完成第一个 future.
                Poll::Pending => return Poll::Pending,
            };
        }
        // 现在第一个future已经完成，尝试完成第二个
        self.second.poll(wake)
    }
}

trait Future {
    type Output;
    fn poll(
        // 注意到这个从 `&mut self` 到 `Pin<&mut Self>`的更改:
        self: Pin<&mut Self>,
        // 也注意从 `wake: fn()` 到 `cx: &mut Context<'_>`的更改:
        cx: &mut Context<'_>,
    ) -> Poll<Self::Output>;
}

