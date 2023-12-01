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
    pub fn new(ipaddr: &str, port: u16) -> Self {
        let address = format!("{}:{}", ipaddr, port);
        let server = TinyHttpServer::http(address.clone()).expect("Failed to bind to the specified address");
        Self { server, url: address }
    }

    pub fn run_server<F>(&self, f: F)
    where
        F: Fn(&sanoRequest::Request) -> SaonResponse
    {
        for mut request in self.server.incoming_requests() {
            //getContentType(&request);
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
/*

use serde_json::Value;
use serde_urlencoded;

fn handle_request(mut request: Request) {
    let content_type = request.headers().iter().find(
        |h| h.field == HeaderField::from_str("Content-Type").unwrap()
    );

    match content_type {
        Some(header) if header.value.eq("application/json") => {
            // Handle JSON request
            handle_json_request(request);
        }
        /*
        Some(header) if header.value.eq("application/x-www-form-urlencoded") => {
            // Handle form request
            handle_form_request(request);
        } */
        _ => {
            // Handle other content types or no content type
            panic!("OH NO")
            //let response = Response::from_string("Unsupported Content-Type")
                //.with_status_code(StatusCode::from(415));
            //request.respond(response).expect("Failed to send response");
        }
    }
}

fn handle_json_request(mut request: Request) {
    let mut buffer = Vec::new();
    request
        .as_reader()
        .read_to_end(&mut buffer)
        .expect("Failed to read request body");

    let body = String::from_utf8_lossy(&buffer);

    // Parse JSON
    match serde_json::from_str::<Value>(&body) {
        Ok(json) => {
            // Handle the parsed JSON
            println!("Received JSON: {:?}", json);

            // Send response
            //let response = Response::from_string("JSON received successfully");
            //request.respond(response).expect("Failed to send response");
        }
        Err(e) => {
            // Handle JSON parsing error
            eprintln!("Error parsing JSON: {:?}", e);

            // Send error response
            //let response = Response::from_string("Error parsing JSON")
                //.with_status_code(StatusCode::from(400));
            //request.respond(response).expect("Failed to send response");
        }
    }
}
*/