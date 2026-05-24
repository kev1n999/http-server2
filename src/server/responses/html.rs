use crate::types::{
    response_type::Response,
    status_code::StatusCode,
    content_type::ContentType,
};

use std::net::TcpStream;

pub fn hello_world(stream: &TcpStream) -> Result<(), std::io::Error> {
    let response = Response::build_response(
        &StatusCode::Ok, 
        ContentType::Html, 
        r#"<script>console.log("Hello World! Im Working!");</script>"#
    );
    
    response.send(&stream)
}