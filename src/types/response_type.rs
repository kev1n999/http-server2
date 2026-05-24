use std::collections::HashMap; 

// Http Response Structure to server send
#[derive(Debug)]
pub struct Response {
    pub version: String,
    pub status_code: u16,
    pub reason: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}