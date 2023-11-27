
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
pub struct Request<'a> {
    query_params: Option<&'a HashMap<String, String>>,
    url: &'a str,
    method: &'a Method,
}

impl<'a> Request<'a>{
    pub fn new(url: &'a str, query_params: Option<&'a HashMap<String, String>>, method: &'a Method) -> Self{
        return Self { url, query_params, method }
    }

    pub fn get_method(&self) -> &'a Method { &self.method }

    pub fn get_url(&self) -> &'a str { &self.url }

    pub fn get_qparams(&self) -> Option<&'a HashMap<String, String>> { self.query_params }
}