use std::thread;
use crate::Result;
use super::ThreadPool;

pub struct NaiveThreadPool;

impl ThreadPool for NaiveThreadPool {
    fn new(_threads: u32) -> Result<Self>
        where
            Self: Sized,
    {
        Ok(NaiveThreadPool)
    }
    fn spawn<F>(&self, job: F)
        where
            F: FnOnce() + Send + 'static,
    {
        thread::spawn(job);
    }
}
