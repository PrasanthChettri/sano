mod response ;
mod server ;
mod router ;
mod sano;
mod request;
mod tinytosano;
mod config;

use request::Method;
use serde::{Serialize, Deserialize};

use crate::response::* ;

#[derive(Serialize, Deserialize, Default, Debug)]
struct Number {
    a: i32,
    b: i32
}

fn main() {
    let mut router: router::Router = router::Router::new(
        "localhost", 
    );
    router.register("/numbers/add", Method::GET, |request| {
        if let Ok(Some(number)) = request.get_body::<Number>() {
            return number.a + number.b ;
        }
        return -1;
    });
    let api: sano::Sano = sano::Sano::new("localhost", &router, &7879) ;
    api.run_server() ;
}
