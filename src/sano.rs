use crate::router;
use crate::server;

pub struct Sano {
    pub router: router::Router,
    server: server::Server
}

impl Sano {

    
    pub fn new(ipaddr: &str, port: u16) -> Self {
        let server = server::Server::new(&String::from(ipaddr), port);
        let router = router::Router::new(&server.url.clone());
        Sano { server, router }
    }

    pub fn handle_connection(router: &router::Router, raw_request: Vec<String> ) -> Vec<String> {
        router.route(&raw_request).send()
    }

    pub fn run_server(&self) {
        self.server.run_server(| request | Self::handle_connection(&self.router, request) )
    }

}