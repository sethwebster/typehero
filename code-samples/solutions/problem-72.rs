use tokio::sync::Mutex;
use std::sync::Arc;

struct SafeMutex<T> {
    data: Arc<Mutex<T>>,
}

impl<T> SafeMutex<T> {
    fn new(data: T) -> Self {
        Self {
            data: Arc::new(Mutex::new(data)),
        }
    }

    async fn lock(&self) -> tokio::sync::MutexGuard<'_, T> {
        self.data.lock().await
    }

    fn clone(&self) -> Self {
        Self {
            data: Arc::clone(&self.data),
        }
    }
}
