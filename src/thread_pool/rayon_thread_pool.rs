use std::panic::UnwindSafe;
use super::ThreadPool;

use rayon::{ThreadPool as RT, ThreadPoolBuilder};

pub struct RayonThreadPool {
    pool: RT,
}

impl ThreadPool for RayonThreadPool {
    fn new(threads: u32) -> crate::Result<Self>
        where
            Self: Sized,
    {
        let pool = ThreadPoolBuilder::new().num_threads(threads as usize).build()?;
        Ok(RayonThreadPool { pool })
    }
    fn spawn<F>(&self, job: F)
        where
            F: FnOnce() + Send + UnwindSafe + 'static,
    {
        self.pool.spawn(job);
    }
}
