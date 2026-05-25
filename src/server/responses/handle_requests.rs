use crate::server::responses::response_404::response_404;
use crate::types::request_type::Request;
use crate::server::responses::handle_responses::{request_is_file, response_by_file, response_by_route};

use std::{
    net::{TcpStream},
};

pub fn request_handler(stream: &TcpStream, request: &Request) -> Result<(), std::io::Error> {
    if request_is_file(request) {
        println!("true");
        response_by_file(stream, request);
    } else {
        response_by_route(stream, request);
    }

    response_404(stream)
}