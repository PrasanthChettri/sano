use serde_urlencoded::{de as de_urlencoded } ;
use log::{info, warn} ;
use serde_json::{Result as JsonResult, Value};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::error::Error;

use std::collections::HashMap;
use std::str::FromStr;
pub struct SerializationError(String);


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

    fn deserialize_form_urlencoded<T: DeserializeOwned>(data: &str) -> Result<Option<T>, SerializationError> {
        // Deserialize the form data using serde_urlencoded
        de_urlencoded::from_str(data)
            .map(Some)
            .map_err(|e| {
                dbg!(data);
                SerializationError(e.to_string())
            })
    }
    
    pub fn get_body<T: DeserializeOwned>(&self) -> Result<Option<T>, SerializationError> {
        match &self.body {
            Body::Nil => Ok(None), 
            Body::Form(d) => {
                todo!("THIS DOES NOT WORK FORM DATA IS TRICKYYY");
                match d.split("\r\n\r\n").nth(1) {
                    Some(e) =>  Self::deserialize_form_urlencoded(d),
                    None => Ok(None),
                }
            }
            Body::FormUrlEncoded(d) => Self::deserialize_form_urlencoded(d.as_ref()),
            Body::Json(d) => serde_json::from_str(d).
                                                    map(Some).
                                                    map_err(|e| SerializationError(e.to_string())),
            _ => todo!()
        }
    }

    pub fn get_qparams(&self) -> &HashMap<String, String> { &self.query_params }
}
