use std::ops::Drop;
use std::thread::{self, spawn};

use crossbeam::channel::{bounded, Iter, Receiver, Sender};

use crate::thread_pool::Job;
use crate::Result;

use super::ThreadPool;

pub struct SharedQueueThreadPool {
    sender: Sender<Job>,
}

#[derive(Clone)]
struct ThreadReceiver(Receiver<Job>);

impl ThreadPool for SharedQueueThreadPool {
    fn new(threads: u32) -> Result<Self>
    where
        Self: Sized,
    {
        let (sender, r) = bounded::<Job>(10);
        for _ in 0..threads {
            let receiver = ThreadReceiver(r.clone());
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
        self.sender.send(Box::new(job)).unwrap();
    }
}

impl ThreadReceiver {
    fn iter(&self) -> Iter<Job> {
        self.0.iter()
    }
}

impl Drop for ThreadReceiver {
    fn drop(&mut self) {
        if thread::panicking() {
            let receiver = self.clone();
            spawn(move || {
                for job in receiver.iter() {
                    job();
                }
            });
        }
    }
}
