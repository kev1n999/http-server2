use crate::{parser::request::parse_request, server::responses::handle_requests::request_handler};

use std::{
    io::{Read},
    net::{TcpStream},
};

// function to handle http connections and get the requests
pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0u8; 4064];
    let req_bytes_received: usize = stream.read(&mut buffer).unwrap();
    let str_request = String::from_utf8_lossy(&buffer[..req_bytes_received]);
    let parts_request: Vec<String> = str_request.split("\r\n\r\n").map(|s| s.to_string()).collect();

    if let Some(request) = parse_request(parts_request) {
        println!("New request: {}\n", request);
        request_handler(&stream, &request).unwrap();
    }
}