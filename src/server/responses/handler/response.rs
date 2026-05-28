use crate::server::responses::default::response_404::response_404;
use crate::server::responses::handler::request::request_is_file;
use crate::server::routes::routes::{routes, static_files};

use crate::types::server::context::Context;
use crate::types::{
    headers::method::Method,
};

pub fn response_by_route(ctx: &mut Context) {
    for route in routes() {
        if route.path == ctx.request.path && route.method == ctx.request.method {
            _ = (route.handler)(ctx);
            break;
        }
    }
}

pub fn response_by_file(ctx: &mut Context) {
    for file in static_files() {
        if ctx.request.path == file.path && ctx.request.method == Method::Get {
            file.send(ctx).expect("An error ocurred to send the file!");
        }
    }
}

pub fn response_handler(ctx: &mut Context) -> Result<(), std::io::Error> {
    if request_is_file(&ctx.request) {
        response_by_file(ctx);
    } else {
        response_by_route(ctx);
    }

    response_404(ctx)
}