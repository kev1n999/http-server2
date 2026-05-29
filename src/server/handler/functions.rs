use crate::{parser::{json::parse_json, r#static::parse_static_file}, types::{
    client::request::Request, headers::{content_type::ContentType, status_code::StatusCode}, server::{context::Context, response::Response}
}};

use serde::Deserialize;

// Example a get response to / route
pub fn home(ctx: &mut Context) -> Result<(), std::io::Error> {
    let home_file = parse_static_file("home.html")?;
    let response = Response::new(StatusCode::Ok, ContentType::Html, &home_file);
    ctx.send(response)
}

// Example a get response to /calc route
pub fn calc(ctx: &mut Context) -> Result<(), std::io::Error> {
    let calc_file = parse_static_file("calc.html")?;
    let response = Response::new(StatusCode::Ok, ContentType::Html, &calc_file);
    ctx.send(response)
}

// Example of a post response to /sum route
pub fn sum(ctx: &mut Context) -> Result<(), std::io::Error> {
    let Request { body, .. } = &ctx.request;

    #[derive(Deserialize)]
    struct Sum {
        x: i32,
        y: i32,
    }
    
    let json: Sum = parse_json(&body)?;
    let sum = json.x + json.y;

    let response = Response::new(StatusCode::Ok, ContentType::Text, &format!("{}", sum));
    ctx.send(response)
}