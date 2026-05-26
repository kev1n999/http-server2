use crate::{
    types::route_type::Route,
    types::method_type::Method,
    types::content_type::ContentType,
    types::static_file::StaticFile,
    server::handler::functions::{home, calc, sum},
};

pub fn routes() -> Vec<Route> {
    vec![
        Route::new(Method::Get, "/".to_string(), home),
        Route::new(Method::Get, "/calc".to_string(), calc),
        Route::new(Method::Post, "/sum".to_string(), sum),
    ]
}

pub fn static_files() -> Vec<StaticFile> {
    vec![
        StaticFile::new("/js/calc.js".to_string(), ContentType::JavaScript),
        StaticFile::new("/css/calc.css".to_string(), ContentType::Css),
    ]
}