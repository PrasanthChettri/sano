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
    pub fn new(val: String , r_type: ResponseType, http_status: u16) -> Self{
        Self { val, r_type , http_status }
    }

    pub fn ok(val: &str, r_type: ResponseType) -> Self { Self::new(val.to_string(), r_type, 200) }

    pub fn err(val: &str, r_type: ResponseType) -> Self { Self::new(val.to_string(), r_type, 400) }

    pub fn send_response_body(self) -> String {
        match self.r_type {
                ResponseType::Raw => self.val,
                ResponseType::HTML => {
                    match fs::read_to_string(self.val) {
                        Ok(content) => content, 
                        Err(e) => panic!("FILE NOT FOUND")
                    }
            }
        }

    }
}

pub struct ResponseBldr {
    response: Response,
}

impl ResponseBldr {
    pub fn new() -> Self { ResponseBldr { response:  Response::default() } }

    pub fn val(mut self, val: &str) -> ResponseBldr {
        self.response.val = val.to_string();
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

    pub fn err(self) -> ResponseBldr { self.http_status(400) }

    pub fn give(self) -> Response { self.response }
}