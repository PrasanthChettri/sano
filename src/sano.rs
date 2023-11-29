use crate::response::Response;
use crate::router;
use crate::server;
use crate::request::Request;

pub struct Sano {
    pub router: router::Router,
    server: server::Server
}

impl Sano {

    pub fn new(ipaddr: &str, port: u16) -> Self {
        let server = server::Server::new(&String::from(ipaddr), port);
        let router = router::Router::new(&String::from(ipaddr));
        Sano { server, router }
    }

    pub fn handle_connection(router: &router::Router, request : &Request) -> Response {
        router.route(request)
    }

    pub fn run_server(&self) {
        self.server.run_server(| request | Self::handle_connection(&self.router, request) )
    }

}