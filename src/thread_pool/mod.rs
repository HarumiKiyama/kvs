use crate::Result;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub trait ThreadPool {
    fn new(threads: u32) -> Result<Self>
    where
        Self: Sized;
    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static;
}

mod naive_thread_pool;
mod rayon_thread_pool;
mod shared_queue_thread_pool;

pub use self::naive_thread_pool::NaiveThreadPool;
pub use self::rayon_thread_pool::RayonThreadPool;
pub use self::shared_queue_thread_pool::SharedQueueThreadPool;
