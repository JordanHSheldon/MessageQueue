use std::{borrow::Cow, io::{Read, Write}, net::{TcpListener, TcpStream}, vec};
use queue::Queue;
use serde::{Deserialize, Serialize};
use serde_json::{Result, from_str};

mod queue;

#[derive(Serialize, Deserialize)]
pub struct QueueRequest {
    action: String,
    id: String
}

impl QueueRequest {
    fn new() -> QueueRequest {
        QueueRequest {
        action: String::new(),
        id: String::new()
    }
}
}

fn handle_client(mut stream: TcpStream) -> QueueRequest{
    let mut buffer = [0;1024];
    stream.read(&mut buffer).expect("failed to read message.");
    let request = String::from_utf8_lossy(&buffer[..]);
    let body_start = request.find("\r\n\r\n").unwrap_or(0) + 4;
    let body = &request[body_start..];
    println!("{}",request);
    let mut parsed = body.get(..25).unwrap_or("").to_string();
    let mut QueueRequest = QueueRequest::new();
    match serde_json::from_str::<QueueRequest>(&parsed) {
        Ok(p) => {
            let response = "HTTP/1.1 200 OK\r\n\
            Content-Length: 2\r\n\
            Content-Type: text/plain\r\n\
            \r\n\
            OK".as_bytes();
            stream.write(response).expect("failed to write message");
            QueueRequest = p;
        },
        Err(e) => {println!("{:?}", e);}
    }
    return QueueRequest;
}

fn main(){
    // make queue
    let mut q = queue::Queue::new();

    // create listener to recieve requests
    let listener = TcpListener::bind("127.0.0.1:8888").
        expect("Error binding ports");

    // accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(stream) =>{
                //std::thread::spawn(|| handle_client(stream));
                q.enqueue(handle_client(stream).action.len().try_into().unwrap());
                println!("Queue is:");
                q.view();
            }
            Err(e) =>{
                println!("{}",e)
            }
        }
    }
}