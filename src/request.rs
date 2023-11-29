
use std::collections::HashMap;
use std::str::FromStr;


#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    OPTIONS,
    HEAD,
    TRACE,
    CONNECT,
}

impl FromStr for Method {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "DELETE" => Ok(Method::DELETE),
            "PATCH" => Ok(Method::PATCH),
            "OPTIONS" => Ok(Method::OPTIONS),
            "HEAD" => Ok(Method::HEAD),
            "TRACE" => Ok(Method::TRACE),
            "CONNECT" => Ok(Method::CONNECT),
            _ => Err(format!("Invalid HTTP method: {}", s)),
        }
    }
}


#[derive(Debug, Clone)]
pub struct Request {
    query_params: Option<HashMap<String, String>>,
    url: String,
    body: Option<String>,
    method: Method
}

impl Request{
    pub fn new(url: String, query_params: Option<HashMap<String, String>>, method:  Method, body: Option<String>) -> Self{
        return Self { url, query_params, method, body }
    }

    pub fn get_method(&self) -> &Method { &self.method }

    pub fn get_url(&self) -> &String { &self.url }

    pub fn get_body(&self) -> &Option<String> { &self.body }

    pub fn get_qparams(&self) -> &Option<HashMap<String, String>> { &self.query_params }
}