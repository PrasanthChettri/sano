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
pub enum Body {
    Json(String),
    Form(String),
    FormUrlEncoded(String),
    Text(String),
    Binary(String),
    Nil
}

#[derive(Debug, Clone)]
pub struct Request {
    query_params: HashMap<String, String>,
    url: String,
    body: Body,
    method: Method
}

impl Request{
    pub fn new(url: String, query_params: HashMap<String, String>, method:  Method, body: Body) -> Self{
        return Self { url, query_params, method, body }
    }

    pub fn get_method(&self) -> &Method { &self.method }

    pub fn get_url(&self) -> &String { &self.url }

    pub fn get_raw_body(&self) -> Option<&String> { 
        match &self.body {
            Body::Nil=> None,
            Body::Json(e) 
           | Body::Form(e)
           | Body::FormUrlEncoded(e)
           | Body::Text(e)
           => Some(e),
            _ => todo!("")
        }
    }
    
    pub fn get_body<T: DeserializeOwned>(&self) -> Result<Option<T>> {
        match self.get_raw_body() {
            None => Ok(None), 
            Some(d) => serde_json::from_str(d).map(Some)
        }
    }

    pub fn get_qparams(&self) -> &HashMap<String, String> { &self.query_params }
}