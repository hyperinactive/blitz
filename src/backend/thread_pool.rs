use std::sync::{
    mpsc::{channel, Sender},
    Arc, Mutex,
};

use super::{server::Job, worker::Worker};

pub struct ThreadPool {
    // "storing" threads
    workers: Vec<Worker>,
    sender: Option<Sender<Job>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = channel();
        // multi ownership, thread safe prt to receiver
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender
            // as ref -> Converts from &Option<T> to Option<&T>.
            .as_ref()
            .expect("Execute error. Sender is None.")
            .send(job)
            .expect("Error send a job.");
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // threads do infinite loops listening for messages from the receiver
        // sender needs to be dropped before joining threads, else they stay open
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
