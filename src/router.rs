use std::collections::HashMap;
use crate::response::{ Response, ResponseType } ;


pub struct Router {
    route_registry: RouteRegistry,
}

#[derive(Eq, Hash, PartialEq, Clone)]
pub enum Method {
    GET, 
    POST, 
    PUT,
    DELETE
}

pub struct Route {
    url: String,
    exec: fn() -> Response
}

pub struct Routes {
    routes: Vec<Route>,
}

pub struct RouteRegistry {
    data: HashMap<Method, Routes>
}

// mut stream: TcpStream, f: 
pub fn get_routes_for_method(routes: &Routes, url: &str) -> Response{
    for route in &routes.routes {
        if route.url.eq(url) {
            return (route.exec)() ;
        }
    }
    return Response::err(String::from("404"), ResponseType::Raw, Some(String::from("404")));
}

impl RouteRegistry {
    //route registry is mutable by default
    pub fn new() -> Self{
        RouteRegistry {
            data : HashMap::new()
        }
    }
}

impl Router {
    pub fn new() -> Self {
        Router {
            route_registry: RouteRegistry::new() ,
        }
    }

    pub fn register(&mut self, url: &str, method: Method, exec: fn() -> Response) {
        let route = Route {
            url: String::from(url),
            exec,
        };

        match self.route_registry.data.get_mut(&method) {
            Some(routes) => routes.routes.push(route),
            None => {
                let routes = Routes {
                    routes: vec![route],
                };
                self.route_registry.data.insert(method.clone(), routes);
            }
        }
    }

    pub fn route(&self, http_request: &Vec<String>) -> Response {
        let status_line = &http_request[0];
        let status: Vec<_> = status_line.split(" ").collect();
        let method  = match status[0] {
            "GET" => Method::GET, 
            "POST" => Method::POST, 
            "PUST" => Method::PUT,
            "DELETE" =>Method::DELETE, 
            _ => panic!("OH NO :-(")
        };
        return match self.route_registry.data.get(&method) { 
            Some(routes) => get_routes_for_method(routes, &status[1]) ,
            None => Response::ok(String::from("404"), ResponseType::Raw, None)
        } ;
    }
}

