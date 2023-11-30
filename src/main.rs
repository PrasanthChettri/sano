mod response ;
mod server ;
mod router ;
mod sano;
mod types;
mod request;
mod tinytosano;
use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::response::* ;

#[derive(Serialize, Deserialize, Default, Debug)]
struct Person {
    a: i32,
    b: i32,
}

fn main() {
    let mut api = sano::Sano::new("localhost", 7879) ;
    api.router.register("/calculate", request::Method::GET, |request| {
        let request_data = request.get_body::<Person>();
        let a = request_data.a;
        let b = request_data.b;
        ResponseBldr::new().ok().val((a+b).to_string()).give()
    });

    api.run_server() ;
}



















