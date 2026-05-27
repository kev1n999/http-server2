use crate::server::responses::default::response_404::response_404;
use crate::server::responses::handler::request::request_is_file;
use crate::server::routes::routes::{routes, static_files};
use crate::types::client::request::Request;

use crate::types::{
    headers::method::Method,
};

use std::net::TcpStream;

pub fn response_by_route(stream: &TcpStream, request: &Request) {
    for route in routes() {
        if route.path == request.path && route.method == request.method {
            _ = (route.handler)(&request, &stream);
            break;
        }
    }
}

pub fn response_by_file(stream: &TcpStream, request: &Request) {
    for file in static_files() {
        if request.path == file.path && request.method == Method::Get {
            file.send(stream).expect("An error ocurred to send the file!");
        }
    }
}

pub fn response_handler(stream: &TcpStream, request: &Request) -> Result<(), std::io::Error> {
    if request_is_file(request) {
        response_by_file(stream, request);
    } else {
        response_by_route(stream, request);
    }

    response_404(stream)
}