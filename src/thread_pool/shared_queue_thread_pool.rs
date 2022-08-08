use std::ops::Drop;
use std::thread::{self, spawn, Thread};

use crossbeam::channel::{bounded, Sender};

use crate::thread_pool::Job;
use crate::Result;

use super::ThreadPool;

pub struct SharedQueueThreadPool {
    sender: Sender<Job>,
}

impl ThreadPool for SharedQueueThreadPool {
    fn new(threads: u32) -> Result<Self>
    where
        Self: Sized,
    {
        let (sender, r) = bounded(0);
        for _ in 0..threads {
            let receiver = r.clone();
            spawn(move || {
                for job in receiver.iter() {
                    job();
                }
            });
        }
        Ok(Self { sender })
    }
    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        match self.sender.send(Box::new(job)) {
            Ok(v) => {}
            Err(e) => {}
        };
    }
}

impl Drop for SharedQueueThreadPool {
    fn drop(&mut self) {
        if thread::panicking() {
            let rx = self.clone()
        }
    }
}
