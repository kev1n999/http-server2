use crate::types::{
    method::Method,
    handler::Handler,
};

#[derive(Debug)]
pub struct Route {
    pub method: Method,
    pub path: String,
    pub handler: Handler,
}

impl Route {
    pub fn new(method: Method, path: String, handler: Handler) -> Self {
        Self { method, path, handler }
    }
}