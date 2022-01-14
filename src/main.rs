use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use porthole::ThreadPool;

fn main() {
    /*
        TODO:
        * Read in config file
        * Customize responses
        * Add logging of connections
    */ 

    // Create a threadpool for each port on thier own thread.
    // for port in 1..100 {
    //     thread::spawn( move || {
    //        create_port_server(port); 
    //     });
    // }

    create_port_server(4444);
}

fn create_port_server(port: usize) {
    let target = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(target).unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("Hello, world!");
}

fn handle_connection(mut stream: TcpStream) {
    // Currently sending a 200 OK. Will need to implent different message depending on port
    // Create randomized responses too.
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}