use crate::types::client::request::Request;

use std::net::TcpStream;

pub struct Context<'a> {
    pub request: Request,
    pub stream: &'a mut TcpStream,
}