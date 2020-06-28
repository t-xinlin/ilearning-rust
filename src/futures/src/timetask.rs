// use {
//     std::{
//         future::Future,
//         pin::Pin,
//         sync::{Arc, Mutex},
//         task::{Context, Poll, Waker},
//         thread,
//         time::Duration,
//     },
// };
//
// pub struct TimerFuture {
//     shared_state: Arc<Mutex<SharedState>>,
// }
//
// /// `future`与等待线程之间的共享状态
// struct SharedState {
//     /// 用于判断sleep的时间是不是已经过了
//     completed: bool,
//
//     /// 任务的唤醒者 `TimerFuture` 正在上面运行.
//     /// 线程能够使用这个设置`completed = true`之后去调用
//     /// `TimerFuture`的任务来唤醒, 观察 `completed = true`, 并前进
//     waker: Option<Waker>,
// }
//
// impl Future for TimerFuture {
//     type Output = ();
//     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
//         // 注意shared_state 并去看计时器（timer）是不是已经完成（completed）了
//         let mut shared_state = self.shared_state.lock().unwrap();
//         if shared_state.completed {
//             Poll::Ready(())
//         } else {
//             // 设置唤醒器，以便线程在计时器（timer）完成的时候可以唤醒当前任务
//             // 确定 future已经再一次被轮询了，并且看`completed = true`.
//             //
//             // 这样做一次很诱人，而不是每次都重复克隆唤醒器。
//             // 但是，`TimerFuture`可以在执行程序上的任务之间移动，
//             // 这可能会导致过时的唤醒程序指向错误的任务，
//             // 从而阻止`TimerFuture`正确唤醒。
//             //
//             // 注意：可以使用 `Waker::will_wake`这个函数来检查
//             // 但是为了简单起见，我们忽略了这个。
//             shared_state.waker = Some(cx.waker().clone());
//             Poll::Pending
//         }
//     }
// }
//
// impl TimerFuture {
//     /// 创建一个新的`TimerFuture` 将在提供timeout之后完成
//     pub fn new(duration: Duration) -> Self {
//         let shared_state = Arc::new(Mutex::new(SharedState {
//             completed: false,
//             waker: None,
//         }));
//
//         // 引发新的线程
//         let thread_shared_state = shared_state.clone();
//         thread::spawn(move || {
//             thread::sleep(duration);
//             let mut shared_state = thread_shared_state.lock().unwrap();
//             // 表示计时器已经完成并唤醒最后一个拥有被轮询过的future的任务，如果它存在的话
//             shared_state.completed = true;
//             if let Some(waker) = shared_state.waker.take() {
//                 waker.wake()
//             }
//         });
//
//         TimerFuture { shared_state }
//     }
// }
