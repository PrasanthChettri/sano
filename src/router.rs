use std::{collections::HashMap, string::ParseError};
use url::Url;
use crate::types::*;

use crate::response::{ Response, ResponseType } ;

pub struct Router {
    root_url: String,
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
    exec: Box<dyn Fn(String, HashMap<String, String>) -> Response>,
    query_params: Option<HashMap<String, String>>,
}

pub struct Routes {
    routes: Vec<Route>,
}

pub struct RouteRegistry {
    data: HashMap<Method, Routes>
}

// mut stream: TcpStream, f: 
pub fn get_routes_for_method(routes: &Routes, url: &str, query_params: HashMap<String, String>) -> Response{
    for route in &routes.routes {
        if route.url.eq(url) {
            return (route.exec)(String::from(url)) ;
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
    pub fn new(root_url: &String) -> Self {
        Router {
            route_registry: RouteRegistry::new() ,
            root_url: root_url.clone()
        }
    }

    pub fn register(&mut self, url: &str, method: Method, exec: fn(String, &HashMap<String, String>) -> Response) {
        let mut a: HashMap<String, String> = HashMap::new();
        a.insert(String::from("a"), String::from("1"));
        let route = Route {
            url: String::from(url),
            exec:  Box::new(move | ur: String, params: &HashMap<String, String>| exec(ur, params)),
            query_params: Some(a)
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
        let request_url = &status[1] ;
        let complete_url = String::from(format!("{}{}", self.root_url.as_str(), request_url));
        let binding = Url::parse(&complete_url).unwrap();
        let query_pairs = binding.query_pairs();
        let query_params = query_pairs.fold(
                    HashMap::new(), |mut acc, (key, value)| {
                    acc.insert(key.into_owned(), value.into_owned());
                    acc
        });

        let partial_url = binding.path();
        print!("{}",partial_url);
        print!("________");

        let method  = match status[0] {
            "GET" => Method::GET, 
            "POST" => Method::POST, 
            "PUST" => Method::PUT,
            "DELETE" =>Method::DELETE, 
            _ => panic!("OH NO :-(")
        };
        return match self.route_registry.data.get(&method) { 
            Some(routes) => get_routes_for_method( routes,&partial_url, query_params) ,
            None => Response::ok(String::from("404"), ResponseType::Raw, None)
        } ;
    }
}

