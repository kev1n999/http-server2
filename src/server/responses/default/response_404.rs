use crate::{parser::r#static::parse_static_file, types::{
    headers::{content_type::ContentType, status_code::StatusCode}, server::{context::Context, response::Response}
}};

pub fn response_404(ctx: &mut Context) -> Result<(), std::io::Error> {
    let content = parse_static_file("404.html")?;
    let response = Response::new(
        StatusCode::NotFound,
        ContentType::Html,
        content.as_str(),
    );

    ctx.send(response)
}