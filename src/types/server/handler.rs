use crate::types::{
    client::request::Request,
};

use std::net::TcpStream;

pub type Handler = fn(&Request, &TcpStream) -> Result<(), std::io::Error>;