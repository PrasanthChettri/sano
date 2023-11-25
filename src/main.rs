mod response ;
mod server ;
mod router ;
mod sano;
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
        "/calculate?op=1+1", 
        router::Method::GET,
        | url | {
            Response::ok(url, ResponseType::Raw, None)
        }
    );
    api.run_server() ;
}



















