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
    let names  = vec!["SUII".to_string(), "SUIII".to_string()] ;
    let mut api = sano::Sano::new("localhost", 7879) ;

    api.router.register("/find_name", request::Method::GET, move|request| {
        match request.get_body::<Person>() {
            Ok(Some(person)) => {
                let response = ResponseBldr::new();
                match names.contains(&person.name) {
                    true => response.err().give(),
                    false => response.ok().val("FOUND".to_string()).give(),
                }
            }
            Ok(None) => Response::err("NOT FOUND".to_string(), ResponseType::Raw),
            Err(_) => Response::err("Error parsing request body".to_string(), ResponseType::Raw),
        }

    });
    api.run_server() ;
}



















