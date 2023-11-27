use std::{ collections::HashMap };
use url::Url;

use crate::{response::{ Response, ResponseType }, request::{Method, Request, self}} ;

pub struct Router {
    root_url: String,
    route_registry: RouteRegistry,
}


pub struct Route {
    url: String,
    exec: Box<dyn Fn(&Request) -> Response>,
    //query_params: Option<HashMap<String, String>>,
}

pub struct Routes {
    routes: Vec<Route>,
}

pub struct RouteRegistry {
    data: HashMap<Method, Routes>
}

// mut stream: TcpStream, f: 
pub fn get_routes_for_method(routes: &Routes, request: Request) -> Response{
    for route in &routes.routes {
        if route.url.eq(request.get_url()) {
            return (route.exec)(&request) ;
        }
    }
    return Response::err(String::from("404"), ResponseType::Raw);
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

    pub fn register<F>(&mut self, url: &str, method: Method, exec: F)
    where
    F: Fn(&Request) -> Response  + 'static
    {
        let mut a: HashMap<String, String> = HashMap::new();
        a.insert(String::from("a"), String::from("1"));
        let route = Route {
            url: String::from(url),
            exec: Box::new(move |request| exec(request)),
            //query_params: Some(a),
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
        let complete_url = String::from(format!("http://{}{}", self.root_url.as_str(), request_url));
        let binding = Url::parse(&complete_url) ;
        if binding.is_err() {
            return Response::err(String::from("404"), ResponseType::Raw);
        }
        let binding = binding.unwrap();
        let query_pairs = binding.query_pairs();
        let query_params = query_pairs.fold(
                    HashMap::new(), |mut acc, (key, value)| {
                    acc.insert(key.into_owned(), value.into_owned());
                    acc
        });

        let partial_url = binding.path();


        let method  = match status[0] {
            "GET" => Method::GET, 
            "POST" => Method::POST, 
            "PUST" => Method::PUT,
            "DELETE" =>Method::DELETE, 
            _ => panic!("OH NO :-(")
        };

        let request = Request::new(partial_url, Some(&query_params) , &method);

        return match self.route_registry.data.get(request.get_method()) { 
            Some(routes) => get_routes_for_method(routes, request) ,
            None => Response::err(String::from("404"), ResponseType::Raw)
        } ;
    }
}