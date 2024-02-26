use crate::response::Response;
use crate::router;
use crate::server;
use crate::config ;
use crate::request::Request;
use crate::config::Config;
use std::sync::Arc;

pub struct Sano<'a> {
    pub router: &'a router::Router,
    server: server::Server,
    context: Option<config::Config>
}

impl<'a> Sano<'a>{
    pub fn new(ipaddr: &'static str, router: &'a router::Router,  port: &u16) -> Self {
        let server = server::Server::new(ipaddr, port);
        Sano { server, router, context: None }
    }
    pub fn mount(&mut self, static_root: &'static str) -> () {
        match &mut self.context {
            Some(context) => context.mount(static_root),
            None => self.context = Some(config::Config::new(static_root)),
        }
    }
    pub fn handle_connection(router: &router::Router ,request : &Request) -> Response {
        router.route(request)
    }
    pub fn run_server(&self) {
        self.server.run_server(| request | Self::handle_connection(&self.router, request) );
    }
}
