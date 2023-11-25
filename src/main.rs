mod response ;
mod server ;
mod router ;
mod sano;
mod types;
use std::collections::HashMap;

use crate::response::* ;

fn main() {
    let mut api = sano::Sano::new("localhost", 7879) ;
    /*
    api.router.register(
        "/api",
        router::Method::POST,
        | url | Response::ok(String::from("HI"), ResponseType::Raw, None)
    );
    api.router.register(
        "/hello", 
        router::Method::GET,
        | url | Response::ok(String::from("hi.html"), ResponseType::HTML, None)
    );
    */
    api.router.register(
        "7879/calculate", 
        router::Method::GET,
        | url , query_params: &HashMap<String, String>| {
            Response::ok(url, ResponseType::Raw, None)
        }
    );
    api.run_server() ;
}



















