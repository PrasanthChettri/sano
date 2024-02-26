use std::collections::HashMap;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::{
    Response as SaonResponse,
    request as sanoRequest, sano,
};
use crate::tinytosano::*;

use tiny_http::{
    Server as TinyHttpServer,
    Request as TinyHttpRequest,
    Response as TinyHttpResponse,
    StatusCode, HeaderField
};


pub struct Server {
    server: Arc<TinyHttpServer>,
    pub url: String,
}

impl Server{
    pub fn new(ipaddr: &str, port: &u16) -> Self {
        let address = format!("{}:{}", ipaddr, port);
        let server = Arc::from(
                TinyHttpServer::http(&address).expect("Failed to bind to the specified address")
            );
        Self { server, url: address }
    }

    pub fn run_server<'a, F>(&self, f: F)
    where
        F: Fn(&sanoRequest::Request) -> SaonResponse //+ Send + Sync + 'static
    {
         for mut request in self.server.incoming_requests() {
            let sano_request = match serialize_from_tiny_http(&mut request){
                Ok(e) => e ,
                Err(e) => panic!("EE"),
            };
            let response = f(&sano_request) ;
            let tiny_response = respond_with_tiny_http(response);
            let _ = request.respond(tiny_response);
        }
        //let f_arc = Arc::new(f) ;
        //let server_clone = Arc::new(&self.server);
        //let mut guards = Vec::with_capacity(4);
        //for i in (0 .. 4) {
            //let server_clone = self.server.clone();
            //let f_clone = f_arc.clone() ;
            //let guard = thread::spawn(move || //{
                //loop //{
                    //let mut rq = server_clone.recv().unwrap();
                    //let sano_request = match serialize_from_tiny_http(&mut rq//){
                            //Ok(e) => //e ,
                            //Err(e) => panic!("EE"),
                    //};
                    //let response = f_clone(&sano_request) ;
                    //dbg!(response);
               // }
            //});
            //guards.push(guard);
        //}
        //for i in guards {
            //i.join();
        //}
    }
}
