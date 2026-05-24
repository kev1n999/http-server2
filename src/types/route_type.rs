use crate::types::{
    method_type::Method,
    handler_type::Handler,
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