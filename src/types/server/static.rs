use crate::types::headers::content_type::ContentType;
use crate::parser::r#static::parse_static_file;
use crate::types::server::context::Context;
use crate::types::server::response::Response;
use crate::types::headers::status_code::StatusCode;

#[derive(Debug)]
pub struct StaticFile {
    pub path: String,
    pub content_type: ContentType,
}

impl StaticFile {
    pub fn new(path: String, content_type: ContentType) -> Self {
        Self { path, content_type }
    }
    pub fn send(&self, ctx: &mut Context) -> Result<(), std::io::Error> {
        let content = parse_static_file(&self.path)?;
        let response = Response::new(StatusCode::Ok, self.content_type, &content);
        ctx.send(response)
    }
}