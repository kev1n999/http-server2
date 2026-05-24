use crate::types::{
    response_type::Response,
    status_code::StatusCode,
    content_type::ContentType,
};

use std::net::TcpStream;

pub fn test_json(stream: &TcpStream) -> Result<(), std::io::Error> {
    let response = Response::build_response(
        &StatusCode::Ok, 
        ContentType::Json, 
        r#"{ "msg": "im working!" }"#);

    response.send(stream)
}