// #![feature(arbitrary_self_types, async_await, futures_api, pin)]

use {
    futures::future::FutureObj,
    std::{
        future::Future,
        pin::Pin,
        sync::{Arc, Mutex},
        sync::mpsc::{sync_channel, SyncSender, Receiver},
        task::{
            local_waker_from_nonlocal,
            Poll, Wake,
            Context, Waker,
        },
        thread,
        time::Duration,
    },
};


pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

/// `future`与等待线程之间的共享状态
struct SharedState {
    /// 用于判断sleep的时间是不是已经过了
    completed: bool,

    /// 任务的唤醒者 `TimerFuture` 正在上面运行.
    /// 线程能够使用这个设置`completed = true`之后去调用
    /// `TimerFuture`的任务来唤醒, 观察 `completed = true`, 并前进
    waker: Option<Waker>,
}

impl Future for TimerFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // 注意shared_state 并去看计时器（timer）是不是已经完成（completed）了
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            Poll::Ready(())
        } else {
            // 设置唤醒器，以便线程在计时器（timer）完成的时候可以唤醒当前任务
            // 确定 future已经再一次被轮询了，并且看`completed = true`.
            //
            // 这样做一次很诱人，而不是每次都重复克隆唤醒器。
            // 但是，`TimerFuture`可以在执行程序上的任务之间移动，
            // 这可能会导致过时的唤醒程序指向错误的任务，
            // 从而阻止`TimerFuture`正确唤醒。
            //
            // 注意：可以使用 `Waker::will_wake`这个函数来检查
            // 但是为了简单起见，我们忽略了这个。
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TimerFuture {
    /// 创建一个新的`TimerFuture` 将在提供timeout之后完成
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        // 引发新的线程
        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            // 表示计时器已经完成并唤醒最后一个拥有被轮询过的future的任务，如果它存在的话
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                waker.wake()
            }
        });

        TimerFuture { shared_state }
    }
}

/// Task executor that receives tasks off of a channel and runs them.
struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

/// `Spawner` spawns new futures onto the task channel.
#[derive(Clone)]
struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

/// A future that can reschedule itself to be polled using a channel.
struct Task {
    // In-progress future that should be pushed to completion
    //
    // The `Mutex` is not necessary for correctness, since we only have
    // one thread executing tasks at once. However, `rustc` isn't smart
    // enough to know that `future` is only mutated from one thread,
    // so we use it in order to provide safety. A production executor would
    // not need this, and could use `UnsafeCell` instead.
    future: Mutex<Option<FutureObj<'static, ()>>>,

    // Handle to spawn tasks onto the task queue
    task_sender: SyncSender<Arc<Task>>,
}

fn new_executor_and_spawner() -> (Executor, Spawner) {
    // Maximum number of tasks to allow queueing in the channel at once.
    // This is just to make `sync_channel` happy, and wouldn't be present in
    // a real executor.
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
    (Executor { ready_queue }, Spawner { task_sender })
}

impl Spawner {
    fn spawn(&self, future: impl Future<Output=()> + 'static + Send) {
        let future_obj = FutureObj::new(Box::new(future));
        let task = Arc::new(Task {
            future: Mutex::new(Some(future_obj)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("too many tasks queued");
    }
}

impl Wake for Task {
    fn wake(arc_self: &Arc<Self>) {
        // Implement `wake` by sending this task back onto the task channel
        // so that it will be polled again by the executor.
        let cloned = arc_self.clone();
        arc_self.task_sender.send(cloned).expect("too many tasks queued");
    }
}

impl Executor {
    fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            let mut future_slot = task.future.lock().unwrap();
            // Take the future, and if it has not yet completed (is still Some),
            // poll it in an attempt to complete it.
            if let Some(mut future) = future_slot.take() {
                // Create a `LocalWaker` from the task itself
                let lw = local_waker_from_nonlocal(task.clone());
                if let Poll::Pending = Pin::new(&mut future).poll(&lw) {
                    // We're not done processing the future, so put it
                    // back in its task to be run again in the future.
                    *future_slot = Some(future);
                }
            }
        }
    }
}

fn main() {
    let (executor, spawner) = new_executor_and_spawner();
    spawner.spawn(async {
        println!("howdy!");
        // Wait for our timer future to complete after two seconds.
        // await!(TimerFuture::new(Duration::new(2, 0)));
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("done!");
    });
    executor.run();
}
