use std::collections::HashMap;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

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
    server: TinyHttpServer,
    pub url: String,
}

impl Server {
    pub fn new(ipaddr: &str, port: &u16) -> Self {
        let address = format!("{}:{}", ipaddr, port);
        let server = TinyHttpServer::http(&address).expect("Failed to bind to the specified address");
        Self { server, url: address }
    }

    pub fn run_server<F>(&self, f: F)
    where
        F: Fn(&sanoRequest::Request) -> SaonResponse
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
    }
}