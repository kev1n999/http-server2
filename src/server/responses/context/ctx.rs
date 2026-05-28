use crate::types::{client::request::Request, server::{context::Context, response::Response}};

use std::io::Write;
use std::net::TcpStream;

impl<'a> Context<'a> {
    pub fn new(request: Request, stream: &'a mut TcpStream) -> Self {
        Self { request, stream }
    }
    pub fn send(&mut self, response: Response) -> Result<(), std::io::Error> {
        self.stream.write_all(&response.to_bytes())
    }
}