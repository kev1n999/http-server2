use crate::{
    types::route_type::Route,
    types::method_type::Method,
    server::handler::functions::home,
};

pub fn routes() -> Vec<Route> {
    vec![
        Route::new(Method::Get, "/".to_string(), home),
    ]
}