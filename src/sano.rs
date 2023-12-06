use crate::response::Response;
use crate::router;
use crate::server;
use crate::config ;
use crate::request::Request;
use crate::config::Config;

pub struct Sano<'a> {
    pub router: router::Router<'a>,
    server: server::Server,
    context: Option<config::Config>
}

impl<'a> Sano<'a> {

    pub fn new(ipaddr: &'static str, port: &u16) -> Self {
        let server = server::Server::new(ipaddr, port);
        let router = router::Router::new(ipaddr);
        Sano { server, router, context: None }
    }
    pub fn mount(&mut self, static_root: &'static str) -> () {
        match &mut self.context {
            Some(context) => context.mount(static_root),
            None => self.context = Some(config::Config::new(static_root)),
        }
    }

    pub fn handle_connection(router: &router::Router, request : &Request) -> Response {
        router.route(request)
    }

    pub fn run_server(&self) {
        self.server.run_server(| request | Self::handle_connection(&self.router, request) )
    }

}