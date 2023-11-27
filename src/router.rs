use std::{ collections::HashMap, str::FromStr };
use url::Url;
use tiny_http::{
    Request as TinyHttpRequest,
};

use crate::{response::{ Response, ResponseType, ResponseBldr }, request::{Method, Request, self}} ;

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
    return ResponseBldr::new().http_status(404).r_type(ResponseType::Raw).give()
}

impl RouteRegistry {
    //route registry is mutable
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
        let route = Route {
            url: String::from(url),
            exec: Box::new(move |request| exec(request)),
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

    pub fn route(&self, http_request: &TinyHttpRequest) -> Response {
        let complete_url = format!("http://{}{}", self.root_url, http_request.url());

        if let Ok(binding) = Url::parse(&complete_url) {
            let query_params: HashMap<_, _> = binding.query_pairs().into_owned().collect();
            let partial_url = binding.path();
            let method = Method::from_str(http_request.method().as_str());

            if let Ok(method) = method {
                let request = Request::new(partial_url, Some(&query_params), &method);
                if let Some(routes) = self.route_registry.data.get(request.get_method()) {
                    return get_routes_for_method(routes, request);
                }
            }
            return ResponseBldr::new().http_status(404).r_type(ResponseType::Raw).give()
        }
        return Response::err(String::from(""), ResponseType::Raw);
    }
}