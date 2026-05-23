use std::collections::HashMap; 

use crate::types::method_type;

#[derive(Debug)]
pub struct Request {
    pub version: String,
    pub path: String,
    pub method: method_type::Method,
    pub headers: HashMap<String, String>,
    // pub body: Vec<u8>,
}