use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

type BoxFuture = Pin<Box<dyn Future<Output = ()> + Send>>;

struct Coroutine {
    future: BoxFuture,
    waker: Option<Waker>,
}

struct Scheduler {
    queue: VecDeque<Coroutine>,
}

impl Scheduler {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    fn spawn(&mut self, future: BoxFuture) {
        self.queue.push_back(Coroutine {
            future,
            waker: None,
        });
    }

    fn run(&mut self) {
        while let Some(mut coroutine) = self.queue.pop_front() {
            let waker = futures::task::noop_waker();
            let mut context = Context::from_waker(&waker);

            match coroutine.future.as_mut().poll(&mut context) {
                Poll::Ready(_) => {
                    // Task completed
                }
                Poll::Pending => {
                    // Re-queue for next iteration
                    self.queue.push_back(coroutine);
                }
            }
        }
    }
}

// Usage
async fn task1() {
    println!("Task 1 yielding");
}

async fn task2() {
    println!("Task 2 yielding");
}
