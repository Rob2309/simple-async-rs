use std::{
    rc::Rc,
    task::{RawWaker, RawWakerVTable, Waker},
};

use crate::task::Task;

pub(crate) fn from_task(task: Rc<Task>) -> Waker {
    let raw = Rc::into_raw(task);
    let raw_waker = RawWaker::new(raw.cast(), &WAKER_VTABLE);
    unsafe { Waker::from_raw(raw_waker) }
}

const WAKER_VTABLE: RawWakerVTable =
    RawWakerVTable::new(waker_clone, waker_wake, waker_wake_by_ref, waker_drop);

/// This function has to clone a given [`Waker`].
/// It is called when `waker.clone()` is called.
fn waker_clone(ptr: *const ()) -> RawWaker {
    // Reconstruct an Rc from the raw pointer contained in the waker
    let rc = unsafe { Rc::<Task>::from_raw(ptr.cast()) };
    // Clone the given Rc to add a reference to it
    let clone = Rc::clone(&rc);
    // Forget the Rc as the cloned waker still exists and has to hold a reference count
    std::mem::forget(rc);
    RawWaker::new(Rc::into_raw(clone).cast(), &WAKER_VTABLE)
}

/// This function has to drop a given [`Waker`].
/// It is called when a [`Waker`] object is dropped.
fn waker_drop(ptr: *const ()) {
    unsafe {
        // ensure that the Rc corresponding to the pointer in the waker is dropped to reduce the ref count
        Rc::<Task>::from_raw(ptr.cast());
    }
}

/// This function has to wake a [`Task`] and drop the given [`Waker`].
/// It is called when `waker.wake()` is called.
fn waker_wake(ptr: *const ()) {
    let rc = unsafe { Rc::<Task>::from_raw(ptr.cast()) };
    let task_queue = rc.task_queue.clone();
    task_queue.send(rc).unwrap();
    // Moving the rc into the sender effectively drops the waker while reusing the Rc
}

/// This function has to wake a [`Task`] without dropping the given [`Waker`].
/// It is called when `waker.wake_by_ref()` is called.
fn waker_wake_by_ref(ptr: *const ()) {
    let rc = unsafe { Rc::<Task>::from_raw(ptr.cast()) };
    // Cloning the rc ensure the refcount is increased
    let rc2 = Rc::clone(&rc);
    rc.task_queue.send(rc2).unwrap();
    // Since the waker is not dropped, we need to forget the original Rc to ensure it's refcount stays as expected.
    std::mem::forget(rc);
}
