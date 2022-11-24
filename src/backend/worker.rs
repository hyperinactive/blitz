use std::sync::mpsc::{self};
use std::sync::{Arc, Mutex};
use std::thread;

use super::server::Job;

pub struct Worker {
    pub id: usize,
    // because of drop trait being implemented we call join on this thread
    // since thread takes ownership this must be Option so "take" method can be called to take ownership
    pub thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            loop {
                // receive a job from sender
                let message = receiver
                    // lock shouldn't fail, if it does, some other thread shit the bed
                    .lock()
                    .expect(&format!(
                        "Job mutex unlock error. Job mutex maybe in a poisoned state. Worker {id}"
                    ))
                    .recv();

                match message {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");
                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
