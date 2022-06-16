use std::{
    future::Future,
    rc::Rc,
    sync::mpsc::{channel, Receiver, Sender},
};

use crate::task::Task;

/// A struct that can be used to execute multiple async Tasks on a single thread.
pub struct Executor {
    task_queue: Receiver<Rc<Task>>,
    task_sender: Sender<Rc<Task>>,
}

impl Executor {
    pub fn new() -> Self {
        let (task_sender, task_queue) = channel();
        Self {
            task_queue,
            task_sender,
        }
    }

    /// Enqueues a future into the executor.
    /// It will be run to completion when [`run()`](Self::run()) is called.
    pub fn spawn(&mut self, future: impl Future<Output = ()> + 'static) {
        let task = Task::new(future, self.task_sender.clone());
        self.task_sender.send(Rc::new(task)).unwrap();
    }

    /// Runs the executor until all tasks are completed.
    /// After the last task is completed, the thread will exit.
    pub fn run(self) -> ! {
        // Drop our own copy of the task sender to ensure task_queue.recv() returns an error when the last Task has finished.
        drop(self.task_sender);

        loop {
            let task = match self.task_queue.recv() {
                Ok(task) => task,
                Err(_) => std::process::exit(0),
            };

            let _ = task.poll();
        }
    }
}

impl Default for Executor {
    fn default() -> Self {
        Self::new()
    }
}
