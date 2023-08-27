use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

pub fn socket_handler(socket_addr: String) {
    let listener = TcpListener::bind(socket_addr).expect("Failed to bind");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    println!("Client connected from: {}", stream.peer_addr().unwrap());
    stream.write_all("Hello, client!".as_bytes()).unwrap();
    println!("Client disconnected");
}
