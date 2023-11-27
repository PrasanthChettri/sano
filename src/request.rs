use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub enum Method {
    GET, 
    POST, 
    PUT,
    DELETE
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