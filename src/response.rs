	
use std::fs ;

#[derive(Debug, Clone, Copy)]
pub enum ResponseType {
    HTML, 
    Raw
}

impl Default for ResponseType {
    fn default() -> Self {
        ResponseType::Raw
    }
}

#[derive(Debug)]
pub struct Response{
    pub val: String,
    pub r_type: ResponseType,
    pub http_status: u16
}

impl Default for Response{
    fn default() -> Self {
        Self {
            val: "".to_string(),
            r_type: ResponseType::default(),
            http_status: 200
        }
    }
}


impl Response {
    pub fn new(val: String, r_type: ResponseType, http_status: u16) -> Self{
        Self { val, r_type , http_status }
    }

    pub fn ok(val: String, r_type: ResponseType) -> Self { Self::new(val, r_type, 200) }

    pub fn err(val: String, r_type: ResponseType) -> Self { Self::new(val, r_type, 403) }

    pub fn send(self) -> Vec<String> {
            let status = (self.http_status).to_string();
            let response = match &self.r_type {
                ResponseType::HTML => Self::send_html_response(&self.val),
                ResponseType::Raw => self.val,
            };
            vec![response, status]
    }

    pub fn send_html_response(fname :&str) -> String {
        match fs::read_to_string(fname) {
            Ok(content) => content, 
            Err(e) => panic!("FILE NOT FOUND")
        }
    }
}

pub struct ResponseBldr {
    response: Response,
}

impl ResponseBldr {
    pub fn new() -> Self { ResponseBldr { response:  Response::default() } }

    pub fn val(mut self, val: String) -> ResponseBldr {
        self.response.val = val;
        self
    }

    pub fn r_type(mut self, r_type: ResponseType) -> ResponseBldr {
        self.response.r_type = r_type;
        self
    }

    pub fn http_status(mut self, http_status: u16) -> ResponseBldr {
        self.response.http_status = http_status;
        self
    }

    pub fn ok(self) -> ResponseBldr { self.http_status(200) }

    pub fn err(self) -> ResponseBldr { self.http_status(403) }

    pub fn give(self) -> Response { self.response }
}

fn main(){
    let a = ResponseBldr::new()
        .val("HI".to_string())
        .r_type(ResponseType::HTML)
        .http_status(200)
        .give();
    dbg!("{}", a);
}