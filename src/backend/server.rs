use std::thread::available_parallelism;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use super::thread_pool::ThreadPool;

pub type Job = Box<dyn FnOnce() + Send + 'static>;

// TODO: rust handbook's setup
// needs execute sql code

// maybe do the graphql type of stuff
// accept only post requests
// read body and figure out what to execute
pub struct Server {}

impl Server {
    pub fn new(address: &str) {
        let listener = TcpListener::bind(address).unwrap();
        // number of cpu threads on a machine
        let default_worker_number = available_parallelism().unwrap().get();

        let pool = ThreadPool::new(default_worker_number);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    pool.execute(|| {
                        Server::handle_connection(stream);
                    });
                }
                Err(e) => {
                    println!("Connection failed. {}", e.kind().to_string())
                }
            }
        }
    }

    fn handle_connection(mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        stream
            .read(&mut buffer)
            .expect("Failed reading from stream buffer.");

        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

        let contents = String::from("get, content");
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
