use crate::{
    types::route_type::Route,
    types::method_type::Method,
    server::handler::functions::{home, calc, sum},
};

pub fn routes() -> Vec<Route> {
    vec![
        Route::new(Method::Get, "/".to_string(), home),
        Route::new(Method::Get, "/calc".to_string(), calc,),
        Route::new(Method::Post, "/sum".to_string(), sum,),
    ]
}