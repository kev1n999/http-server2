use crate::types::server::context::Context;

pub type Handler = fn(&mut Context) -> Result<(), std::io::Error>;