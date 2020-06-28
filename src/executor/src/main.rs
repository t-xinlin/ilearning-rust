#![feature(arbitrary_self_types, async_await, await_macro, futures_api, pin)]

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
        },
    },
};

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
        await!(TimerFuture::new(Duration::new(2, 0)));
        println!("done!");
    });
    executor.run();
}
