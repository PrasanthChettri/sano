use std::{ collections::HashMap, str::FromStr };

use std::fmt;
use url::Url;
use crate::request::{Request, Method};

use crate::response::{ Response, Body, ResponseBldr, ResponseSerializable };

pub struct Router {
    root_url: &'static str,
    route_registry: RouteRegistry<'static>,
}


enum RouteType {
    Leaf,
    Endpoint(Box<dyn Fn(&Request) -> Response>)
}

impl fmt::Debug for RouteType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RouteType::Leaf => write!(f, "Leaf"),
            RouteType::Endpoint(_) => write!(f, "Endpoint(<function>)"),
        }
    }
}

#[derive(Debug)]
pub struct Route<'a> {
    rType: RouteType,
    children: HashMap<&'a str, Route<'a>>,
}
unsafe impl Send for Route<'_> {}
unsafe impl Sync for Route<'_> {}

impl<'a> Default for Route<'a> {
    fn default() -> Self {
        Route {
            children : HashMap::new(),
            rType: RouteType::Leaf,
        }
    }
}


//trie stores the segmented url
#[derive(Debug)]
pub struct RouteRegistry<'a> {
    data: HashMap<Method, Route<'a>>
}

impl<'a> RouteRegistry<'a> {
    //route registry is mutable
    pub fn new() -> Self{
        RouteRegistry {
            data : HashMap::new()
        }
    }
    pub fn register(&mut self, url: &'static str ,rType: RouteType, method: Method) -> () {
        let routes = self.data.entry(method).or_default();
        let mut current = routes ;
        for sec in url.split("/") {
            current = current.children.entry(sec).or_default() ;
        }
        current.rType = rType;
    }

    pub fn find(&self, url: &String, method: &Method) -> Option<&Route> {
        let routes = self.data.get(method)?;
        let mut current = routes ;
        for sec in url.split("/"){
            current = current.children.get(sec)?;
        }
        Some(&current)
    }
}


impl Router {
    pub fn new(root_url: &'static str) -> Self {
        Router {
            route_registry: RouteRegistry::new() ,
            root_url: root_url
        }
    }

    pub fn register<F, T>(&mut self, url: &'static str, method: Method, exec: F)
    where
    F: Fn(&Request) -> T  + 'static,
    T: ResponseSerializable
    {
        let rType = RouteType::Endpoint(
                Box::new(move |request|
                {
                    let response : T  = exec(request);
                    response.serialze()
                })
            );
        self.route_registry.register(url, rType, method);

    }

    pub fn execute(route: &Route, request: &Request) -> Response {
        match &route.rType {
            RouteType::Leaf => ResponseBldr::new().http_status(404).give(),
            RouteType::Endpoint(f) => f(request)
        }
    }

    pub fn route(&self, http_request: &Request) -> Response {
        if let Some(route) = self.route_registry.find(
            http_request.get_url(), http_request.get_method()
        ) {
            
            return Self::execute(route, &http_request);
        }
        return ResponseBldr::new().http_status(404).give()
    }
}
