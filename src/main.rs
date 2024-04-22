use std::{io::Read,io::Write, net::{TcpListener, TcpStream}};

mod queue;

pub struct QueueRequest {
    action: String,
    id: String
}

fn handle_client(mut stream: TcpStream){
    let mut buffer = [0;1024];
    stream.read(&mut buffer).expect("failed to read message.");

    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Request: {}",request);
    let response = "Recieved".as_bytes();
    stream.write(response).expect("failed to write message");
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
                std::thread::spawn(|| handle_client(stream));
                q.enqueue(1);
            }
            Err(e) =>{
                println!("{}",e)
            }
        }
    }
}