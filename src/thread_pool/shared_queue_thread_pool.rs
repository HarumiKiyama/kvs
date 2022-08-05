use std::panic::{catch_unwind, UnwindSafe};
use std::thread::spawn;

use crossbeam::channel::{bounded, Sender};

use crate::Result;
use crate::thread_pool::ThreadPoolMessage;

use super::ThreadPool;

pub struct SharedQueueThreadPool {
    threads: u32,
    sender: Sender<ThreadPoolMessage>,
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
                for msg in receiver.iter() {
                    match msg {
                        ThreadPoolMessage::RunJob(job) => {
                            match catch_unwind(job) {
                                Ok(..) => {}
                                Err(e) => { println!("{:?}", e) }
                            }
                        }
                        ThreadPoolMessage::Shutdown => {}
                    }
                }
            });
        }
        Ok(Self { threads, sender })
    }
    fn spawn<F>(&self, job: F)
        where
            F: FnOnce() + Send + UnwindSafe + 'static,
    {
        self.sender.send(ThreadPoolMessage::RunJob(Box::new(job))).unwrap();
    }
}
