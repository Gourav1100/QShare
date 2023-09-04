use std::collections::HashMap;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::RwLock;
use std::thread;

use lazy_static::lazy_static;

lazy_static! {
    static ref PENDING_AUTHORIZATION: RwLock<HashMap<String, TcpStream>> =
        RwLock::new(HashMap::<String, TcpStream>::new());
}

pub fn socket_handler(socket_addr: String, window: tauri::Window) {
    let listener = TcpListener::bind(socket_addr).expect("Failed to bind");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let window_clone = window.clone();
                thread::spawn(move || {
                    handle_client(stream, window_clone);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream, window: tauri::Window) {
    const EXIT_SIGNAL: &[u8] = b"exit";
    println!("Client connected from: {}", stream.peer_addr().unwrap());
    loop {
        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                let received_data = &buffer[..bytes_read];
                if received_data.starts_with(EXIT_SIGNAL) {
                    println!("Exit signal received from client.");
                    break;
                } else {
                    let response: String;
                    let utf8_str = std::str::from_utf8(received_data);
                    let utf8_lossy_string = String::from_utf8_lossy(received_data);
                    if utf8_str.is_err() {
                        response = utf8_lossy_string.to_string();
                    } else {
                        response = utf8_str.unwrap().to_string();
                    }
                    println!("Received {} bytes: {}", bytes_read, response);
                    match response.as_str() {
                        "authorize" => {
                            let config = super::config_handler::get_config();
                            if config.is_err() {
                                stream.write_all(config.err().unwrap().as_bytes()).unwrap();
                            } else {
                                let _request_address = stream
                                    .peer_addr()
                                    .unwrap()
                                    .to_string()
                                    .split(":")
                                    .collect::<Vec<&str>>()[0]
                                    .to_string();
                                let mut config_unwrapped = config.unwrap();
                                let approved_list: Vec<bool> = config_unwrapped
                                    .approved
                                    .iter()
                                    .filter_map(|(key, &value)| {
                                        let key_value = key.as_str().to_string();
                                        if key_value == _request_address {
                                            Some(value)
                                        } else {
                                            None
                                        }
                                    })
                                    .collect();
                                if approved_list.len() == 1 && approved_list[0] {
                                    stream.write_all("approved".as_bytes()).unwrap();
                                } else {
                                    config_unwrapped
                                        .approved
                                        .insert(_request_address.clone(), false);
                                    super::config_handler::set_config(config_unwrapped).unwrap();
                                    let stream_clone = stream.try_clone().unwrap();
                                    let mut map = PENDING_AUTHORIZATION.write().unwrap();
                                    map.insert(_request_address.clone(), stream_clone);
                                    drop(map);
                                    window.emit("authorize", _request_address.clone()).unwrap();
                                }
                            }
                        }
                        _ => {
                            stream.write_all("invalid request".as_bytes()).unwrap();
                        }
                    }
                }
            }
            Err(e) => {
                println!("Error reading from socket: {}", e);
                break;
            }
        }
    }
    let mut map = PENDING_AUTHORIZATION.write().unwrap();
    map.remove(&stream.peer_addr().unwrap().to_string());
    println!("Client disconnected");
}

pub fn respond_and_update_authentication_status(status: bool, ip_address: String) -> bool {
    let config = super::config_handler::get_config();
    if config.is_err() {
        return false;
    } else {
        let mut config_unwrapped = config.unwrap();
        config_unwrapped.approved.remove(ip_address.as_str());
        config_unwrapped.approved.insert(ip_address.clone(), status);
        super::config_handler::set_config(config_unwrapped).unwrap();
        let mut map = PENDING_AUTHORIZATION.write().unwrap();
        if let Some(mut stream) = map.remove(&ip_address.clone()) {
            let mut response = "denied";
            if status {
                response = "approved";
            }
            stream.write_all(response.to_string().as_bytes()).unwrap();
        }
    }
    return true;
}
