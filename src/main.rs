mod response ;
mod server ;
mod router ;
mod sano;
use crate::response::* ;

fn main() {
    let mut api = sano::Sano::new("localhost", 7879) ;
    api.router.register(
        "/api",
        router::Method::POST,
        | | Response::ok(String::from("HI"), ResponseType::Raw, None)
    );
    api.router.register(
        "/hello", 
        router::Method::GET,
        | | Response::ok(String::from("hi.html"), ResponseType::HTML, None)
    );
    api.router.register(
        "/sodu", 
        router::Method::GET,
        | | Response::ok(String::from("this.html"), ResponseType::HTML, None)
    );
    api.run_server() ;
}



















