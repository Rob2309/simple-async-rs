use std::{
    cell::{RefCell, UnsafeCell},
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

        let task_sender = self.task_queue.clone();

        let waker = from_task(self);
        let mut context = Context::from_waker(&waker);

        CURRENT_TASK_SENDER.with(|cell| {
            cell.replace(Some(task_sender));
            let res = pin.poll(&mut context);
            cell.replace(None);
            res
        })
    }

    /// Spawns a new [`Task`] that will run on the same executor as the current [`Task`]
    ///
    /// # Panics
    /// This function panics if it is called from a context outside an executor or an async function.
    pub fn spawn(future: impl Future<Output = ()> + 'static) {
        let task_sender = CURRENT_TASK_SENDER.with(|cell| {
            cell.borrow()
                .as_ref()
                .expect("Task::spawn() called from outside an executor")
                .clone()
        });

        let task_sender2 = task_sender.clone();

        let task = Self::new(future, task_sender);
        task_sender2.send(Rc::new(task)).unwrap();
    }
}

thread_local! {
    /// This variable holds a [`Sender`] that can be used to enqueue a new [`Task`] into the [`Executor`](crate::executor::Executor) that is currently running
    pub(crate) static CURRENT_TASK_SENDER: RefCell<Option<Sender<Rc<Task>>>> = RefCell::new(None);
}
