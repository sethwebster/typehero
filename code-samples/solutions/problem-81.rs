use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

struct LockFreeQueue<T> {
    items: Vec<Option<T>>,
    head: AtomicUsize,
    tail: AtomicUsize,
}

impl<T> LockFreeQueue<T> {
    fn new(capacity: usize) -> Arc<Self> {
        Arc::new(Self {
            items: (0..capacity).map(|_| None).collect(),
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
        })
    }

    fn enqueue(&self, item: T) -> Result<(), T> {
        let tail = self.tail.load(Ordering::Acquire);
        let next_tail = (tail + 1) % self.items.len();

        if next_tail == self.head.load(Ordering::Acquire) {
            return Err(item); // Queue full
        }

        // SAFETY: Single producer assumed or external synchronization required
        unsafe {
            let slot = &self.items[tail] as *const Option<T> as *mut Option<T>;
            *slot = Some(item);
        }

        self.tail.store(next_tail, Ordering::Release);
        Ok(())
    }

    fn dequeue(&self) -> Option<T> {
        let head = self.head.load(Ordering::Acquire);

        if head == self.tail.load(Ordering::Acquire) {
            return None; // Queue empty
        }

        // SAFETY: Single consumer assumed or external synchronization required
        let item = unsafe {
            let slot = &self.items[head] as *const Option<T> as *mut Option<T>;
            (*slot).take()
        };

        let next_head = (head + 1) % self.items.len();
        self.head.store(next_head, Ordering::Release);
        item
    }
}
