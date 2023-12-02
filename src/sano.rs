use crate::response::Response;
use crate::router;
use crate::server;
use crate::request::Request;

pub struct Sano<'a> {
    pub router: router::Router<'a>,
    server: server::Server
}

impl<'a> Sano<'a> {

    pub fn new(ipaddr: &'static str, port: &u16) -> Self {
        let server = server::Server::new(ipaddr, port);
        let router = router::Router::new(ipaddr);
        Sano { server, router }
    }

    pub fn handle_connection(router: &router::Router, request : &Request) -> Response {
        router.route(request)
    }

    pub fn run_server(&self) {
        self.server.run_server(| request | Self::handle_connection(&self.router, request) )
    }

}