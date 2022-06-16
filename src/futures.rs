use std::{
    future::Future,
    task::Poll,
    time::{Duration, Instant},
};

pub struct Timer {
    end: Instant,
}

impl Future for Timer {
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if Instant::now() < self.end {
            let end = self.end;
            let waker = cx.waker().clone();
            std::thread::spawn(move || {
                std::thread::sleep(end - Instant::now());
                waker.wake();
            });

            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}

pub fn sleep(dur: Duration) -> Timer {
    Timer {
        end: Instant::now() + dur,
    }
}
