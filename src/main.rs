mod types;
mod server;
mod headers;

use crate::server::listening;
use std::net::TcpListener;

fn main() {
    let mut stream = TcpListener::bind("127.0.0.1:8000").expect("An error ocurred to bind!");
    for stream in stream.incoming() {
        let stream = stream.unwrap();
        listening::handle_connection(stream);
    }
}