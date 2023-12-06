use std::{ collections::HashMap, str::FromStr };
use url::Url;
use crate::request::{Request, Method};

use crate::response::{ Response, ResponseType, ResponseBldr, ResponseSerializable };

pub struct Router<'a> {
    root_url: &'static str,
    route_registry: RouteRegistry<'a>,
}


pub struct Route<'a> {
    url: &'a str,
    exec: Box<dyn Fn(&Request) -> Response>,
    //query_params: Option<HashMap<String, String>>,
}

pub struct Routes<'a> {
    routes: Vec<Route<'a>>,
}

pub struct RouteRegistry<'a> {
    data: HashMap<Method, Routes<'a>>
}

// mut stream: TcpStream, f: 
pub fn get_routes_for_method(routes: &Routes, request: &Request) -> Response{
    for route in &routes.routes {
        if route.url.eq(request.get_url()) {
            return (route.exec)(&request) ;
        }
    }
    return ResponseBldr::new().http_status(404).r_type(ResponseType::Raw).give()
}

impl<'a> RouteRegistry<'a> {
    //route registry is mutable
    pub fn new() -> Self{
        RouteRegistry {
            data : HashMap::new()
        }
    }
}

impl<'a> Router<'a> {
    pub fn new(root_url: &'static str) -> Self {
        Router {
            route_registry: RouteRegistry::new() ,
            root_url: root_url
        }
    }

    pub fn register<F, T>(&mut self, url: &'a str, method: Method, exec: F)
    where
    F: Fn(&Request) -> T  + 'static,
    T: ResponseSerializable
    {
        let route = Route {
            url: url,
            exec: Box::new(move |request|
                {
                    let response : T  = exec(request);
                    response.serialze()
                }
            ),
        };

        self.route_registry
            .data
            .entry(method)
            .or_insert_with(|| Routes {
                routes: Vec::new(),
            })
            .routes
            .push(route);

    }

    pub fn route(&self, http_request: &Request) -> Response {
        if let Some(routes) = self.route_registry.data.get(http_request.get_method()) {
            return get_routes_for_method(routes, http_request);
        }
        return ResponseBldr::new().http_status(404).r_type(ResponseType::Raw).give()
    }
}