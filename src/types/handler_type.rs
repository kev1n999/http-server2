use crate::types::{
    response_type::Response,
    request_type::Request,
};

use std::net::TcpStream;

pub type Handler = fn(&Request, &TcpStream) -> Result<(), std::io::Error>;