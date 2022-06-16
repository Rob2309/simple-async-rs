use std::{
    cell::UnsafeCell,
    future::Future,
    pin::Pin,
    rc::Rc,
    sync::mpsc::Sender,
    task::{Context, Poll},
};

use crate::waker::from_task;

/// Holds state of a single top-level task that can be run by an executor
pub struct Task {
    /// The actual storage needed to execute the Task's code
    future: UnsafeCell<Box<dyn Future<Output = ()>>>,
    /// A sender that can be used to re-queue this Task to the executor
    pub(crate) task_queue: Sender<Rc<Task>>,
}

impl Task {
    /// Creates a new Task instance
    pub(crate) fn new(
        future: impl Future<Output = ()> + 'static,
        task_queue: Sender<Rc<Self>>,
    ) -> Self {
        Self {
            future: UnsafeCell::new(Box::new(future)),
            task_queue,
        }
    }

    /// Continues running the Task to the end or the next yielding point
    pub(crate) fn poll(self: Rc<Self>) -> Poll<()> {
        // Accessing the UnsafeCell is safe, a Task will only ever be accessed from a single point in code in a single thread.
        let future = unsafe { &mut *self.future.get() }.as_mut();
        // Pinning is safe, Tasks are never moved out of the Rc by the executor
        let pin = unsafe { Pin::new_unchecked(future) };

        let waker = from_task(self);
        let mut context = Context::from_waker(&waker);
        pin.poll(&mut context)
    }
}
