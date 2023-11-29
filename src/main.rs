mod response ;
mod server ;
mod router ;
mod sano;
mod types;
mod request;
mod tinytosano;
use std::collections::HashMap;

use crate::response::* ;

fn main() {
    let mut api = sano::Sano::new("localhost", 7879) ;
    api.router.register("/calculate", request::Method::GET, |request| {
        let query_params : &Option<HashMap<String, String>> = request.get_qparams() ;
        if query_params.is_none() {
            return ResponseBldr::new().err().val("parameters not provided".to_string()).give();
        }
        let query_params = query_params.clone().unwrap();
        let safe_extract = | s : &str, d: &i32 | {
                            query_params.get(s).unwrap_or(&d.to_string()).
                            trim().parse().unwrap_or(0)
        };
        let a :i32= safe_extract("a", &0);
        let b :i32= safe_extract("b", &0);
        ResponseBldr::new().ok().val((a+b).to_string()).give()
    });

    api.run_server() ;
}



















