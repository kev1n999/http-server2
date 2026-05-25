use crate::server::routes::routes::{routes, static_files};
use crate::types::request_type::Request;

use crate::types::{
    method_type::Method,
};

use std::{
    net::TcpStream,
    path::Path,
};

pub fn request_is_file(request: &Request) -> bool {
    let valid_extensions = ["html", "css", "js", "png", "jpg", "jpeg", "svg", "ico"];

    Path::new(&request.path)
        .extension()
        .and_then(|e| e.to_str())
        .is_some_and(|ext| valid_extensions.contains(&ext))
}

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