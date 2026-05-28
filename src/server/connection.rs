use crate::{
    parser::request::parse_request,
    server::responses::handler::response::response_handler, types::server::context::Context,
};

use std::{
    io::{Read},
    net::{TcpStream},
};

// function to handle http connections and get the requests
pub fn handle_connection(stream: &mut TcpStream) {
    let mut buffer = [0u8; 4064];
    let req_bytes_received: usize = stream.read(&mut buffer).unwrap();
    let str_request = String::from_utf8_lossy(&buffer[..req_bytes_received]);
    let parts_request: Vec<String> = str_request.split("\r\n\r\n").map(|s| s.to_string()).collect();

    if let Some(request) = parse_request(parts_request) {
        println!("New request: {}\n", request);
        let mut context = Context::new(request, stream);
        response_handler(&mut context);
    }
}