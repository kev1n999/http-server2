mod types;
mod server;
mod parser;

use crate::server::connection;

use std::net::TcpListener;

fn main() {
    let stream = TcpListener::bind("127.0.0.1:8000").expect("An error ocurred to bind!");
    for stream in stream.incoming() {
        let stream = stream.unwrap();
        connection::handle_connection(stream);
    }
}