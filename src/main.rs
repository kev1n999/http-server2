mod types;
mod server;
mod parser;

use crate::server::connection;

use std::net::TcpListener;

const HOST: &str = "127.0.0.1:8000";

fn main() {
    let stream = TcpListener::bind(HOST).expect("An error ocurred to bind socket!");
    for stream in stream.incoming() {
        let mut stream = stream.unwrap();
        connection::handle_connection(&mut stream);
    }
}