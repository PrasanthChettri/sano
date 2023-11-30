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
    name: String,
}

fn main() {
    let mut names  = vec!["SUII".to_string(), "SUIII".to_string()] ;
    let mut api = sano::Sano::new("localhost", 7879) ;
    api.router.register("/find_name", request::Method::GET, move|request| {
        let data = request.get_body::<Person>();
        let name = data.name ;
        let response = ResponseBldr::new()  ;
        match names.contains(&name) {
            true =>  response.err().give() ,
            false => response.ok().val("FOUND".to_string()).give() ,
        }
    });
    api.run_server() ;
}



















