use std::pin::Pin;
use std::task::Poll;

struct SocketRead<'a> {
    socket: &'a Socket,
}

impl SimpleFuture for SocketRead<'_> {
    type Output = Vec<u8>;

    fn poll(self: Pin<&mut Self>, lw: &LocalWaker) -> Poll<Self::Output> {
        if self.socket.has_data_to_read() {
            // The socket has data-- read it into a buffer and return it.
            Poll::Ready(self.socket.read_buf())
        } else {
            // The socket does not yet have data.
            //
            // Arrange for `wake` to be called once data is available.
            // When data becomes available, `wake` will be called, and the
            // user of this `Future` will know to call `poll` again and
            // receive data.
            self.socket.set_readable_callback(lw);
            Poll::Pending
        }
    }
}

struct IoBlocker {}

struct Event {
    // An ID uniquely identifying the event that occurred and was listened for.
    id: usize,

    // A set of signals to wait for, or which occurred.
    signals: Signals,
}

impl IoBlocker {
    /// Create a new collection of asynchronous IO events to block on.
    fn new() -> Self {}

    /// Express an interest in a particular IO event.
    fn add_io_event_interest(
        &self,
        /// The object on which the event will occur
        io_object: &IoObject,
        /// A set of signals that may appear on the `io_object` for
        /// which an event should be triggered, paried with
        /// an ID to give to events that result from this interest.
        event: Event,
    ) {}

    /// Block until one of the events occurs.
/// This will only trigget
    fn block(&self) -> Event {}
}

fn main() {
    let mut io_blocker = IoBlocker::new();

    io_blocker.add_io_event_interest(
        &socket_1,
        Event { id: 1, signals: READABLE },
    );
    io_blocker.add_io_event_interest(
        &socket_2,
        Event { id: 2, signals: READABLE | WRITABLE },
    );
    let event = io_blocker.block();

    // prints e.g. "Socket 1 is now READABLE" if socket one became readable.
    println!("Socket {:?} is now {:?}", event.id, event.signals);
}

// impl Socket {
// fn set_readable_callback(&self, lw: &LocalWaker) {
// // `local_executor` is a reference to the local executor.
// // this could be provided at creation of the socket, but in practice
// // many executor implementations pass it down through thread local
// // storage for convenience.
// let local_executor = self.local_executor;
//
// // Unique ID for this IO object.
// let id = self.id;
//
// // Store the local waker in the executor's map so that it can be called
// // once the IO event arrives.
// local_executor.event_map.insert(id, lw.clone());
// local_executor.add_io_event_interest(
// &self.socket_file_descriptor,
// Event { id, signals: READABLE },
// );
// }
// }

