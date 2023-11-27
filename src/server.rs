use std::{
};
use tiny_http::{
    Server as TinyHttpServer,
    Request,
    Response,
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
        F: Fn(&Request) -> Vec<String> 
    {
        for request in self.server.incoming_requests() {
            let response = f(&request);
            let status_code = (&response[1]).parse::<u16>().unwrap(); // Parse status code as u16
            let response = &response[0];
            let tiny_response = Response::from_string(response)
                                .with_status_code(status_code);
            let _ = request.respond(tiny_response);
        }
    }
}