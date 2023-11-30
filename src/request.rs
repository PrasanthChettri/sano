use serde_json::{Result, Value};
use serde::{Deserialize, Serialize, de::DeserializeOwned};


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
    NONSTANDARD,
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

    pub fn get_raw_body(&self) -> &Option<String> { &self.body }
    
    pub fn get_body<T: DeserializeOwned>(&self) -> T { json::<T>(&self.body.clone().unwrap_or("".to_string())) }

    pub fn get_qparams(&self) -> Option<&HashMap<String, String>> { 
        match &self.query_params  {
            Some(e) => Some(&e),
            None => None
        }
    }
}

fn json<T: DeserializeOwned>(data: &String)  -> T {
    let v: T = serde_json::from_str(&data).unwrap();
    return v ;
}