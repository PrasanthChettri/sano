mod response ;
mod server ;
mod router ;
mod sano;
mod types;
mod request;
mod tinytosano;
use std::{collections::HashMap, rc::Rc};

use serde::{Serialize, Deserialize};

use crate::response::* ;


#[derive(Serialize, Deserialize, Default, Debug)]
struct Person {
    name: String,
}


fn main() {
    let mut api = sano::Sano::new("localhost", &7879) ;

    let names = vec![ "NAME1", "NAME2"] ;

    api.router.register("/find_name", request::Method::GET, move |request| {

        let ok_response : ResponseBldr = ResponseBldr::new().ok();
        match request.get_body::<Person>() {
            Ok(Some(person)) => 
                match names.contains(&person.name.as_ref()) {
                    true => ok_response.val("FOUND").give(),
                    false => ok_response.val("NOT FOUND").give(),
            }

            Ok(None) => Response::err("NOT DATA", ResponseType::Raw),
            Err(_) => Response::err("Error parsing request body", ResponseType::Raw),
        }

    });
    api.run_server() ;
}