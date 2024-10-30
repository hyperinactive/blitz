use std::sync::{
    mpsc::{channel, Receiver, Sender},
    Arc, Mutex,
};

use super::worker::Worker;

// FnOnce
// instances which impl FnOnce can only be called once
// usually sent to closures which call it once and drop
// +
// Send
// impl Send -> cause we're getting it from the channel
// +
// 'static -> infinite lifetime?
pub type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    // "storing" threads
    workers: Vec<Worker>,
    sender: Option<Sender<Job>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = channel::<Job>();
        // multi ownership, thread safe prt to receiver
        let receiver: Arc<Mutex<Receiver<Job>>> = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    // passing this closure containing fn Server::handle_connection(stream)
    // FnOnce cause we need it run only once -> consume the closure
    pub fn execute<F>(&self, action_to_execute: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // ptr to the action to execute -> send via sender
        let job = Box::new(action_to_execute);
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
