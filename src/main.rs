mod response ;
mod server ;
mod router ;
mod sano;
mod types;
mod request;
mod tinytosano;
mod config;
use std::{collections::HashMap, rc::Rc};

use request::Method;
use serde::{Serialize, Deserialize};

use crate::response::* ;


#[derive(Serialize, Deserialize, Default, Debug)]
struct Person {
    name: String
}


fn main() {
    let mut api: sano::Sano<'static> = sano::Sano::new("localhost", &7879) ;
    api.mount("D:\\") ;


    api.router.register(
        "/find_name", Method::GET, |request| 
    {
        let names = [ "NAME1", "NAME2"].map(String::from) ;

        match request.get_body::<Person>() {
            Ok(Some(person)) => 
                match names.contains(&person.name) {
                    true => "FOUND",
                    false => "NOT FOUND"
            }

            Ok(None) => "NO_DATA",
            Err(_) => "Error parsing request body",
        }

    });
    api.run_server() ;
}