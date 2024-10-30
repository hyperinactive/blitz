use std::collections::HashMap;
use std::net::TcpStream;

use std::io::prelude::*;
use std::{env, error};

pub struct Client {}

impl Client {
    // TODO: send the actual command
    fn send_one_command(command: &str) -> Result<String, Box<dyn error::Error + Send + Sync>> {
        // TODO: hard coded
        let content = String::from(
            "
            Request: GET / HTTP/1.1
            Content-Type: application/json
            Accept: */*
            Host: localhost:8080
            Accept-Encoding: gzip, deflate, br
            Connection: keep-alive
            Content-Length: 93
            {
                \"exec\": \"run create database jojo { create table jiji { create column jaja { } } }\"
            }",
        );

        let mut stream = TcpStream::connect("127:0:0:1:8080").unwrap();
        stream.write_all(content.as_bytes()).unwrap();

        let mut response = String::new();
        let mut limited = stream.take(1024);
        limited.read_to_string(&mut response).unwrap();
        println!("{}", response);

        Ok(response)
    }
    pub fn new() {
        // TODO: assuming the server is listening to 8080
        let mut stream = TcpStream::connect("127.0.0.1:8080");

        Client::send_one_command("hi");
    }
}
