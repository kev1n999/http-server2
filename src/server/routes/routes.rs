use crate::{
    types::route_type::Route,
    types::method_type::Method,
    server::handler::functions::{home, test, sum_two_numbers},
};

pub fn routes() -> Vec<Route> {
    vec![
        Route::new(Method::Get, "/".to_string(), home),
        Route::new(Method::Get, "/test".to_string(), test),
        Route::new(Method::Post, "/calc".to_string(), sum_two_numbers,)
    ]
}