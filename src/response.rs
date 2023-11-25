use std::fs ;

#[derive(Debug, Clone, Copy)]
pub enum ResponseType {
    HTML, 
    Raw
}

pub struct Response{
    pub val: String,
    pub r_type: ResponseType,
    pub http_status: Result<String, String>,
}


impl Response {
    fn new(val: String, r_type: ResponseType, http_status: Result<String,String>) -> Self{
        Self {
                val,
                r_type ,
                http_status: http_status,
            }
    }

    pub fn ok(val: String, r_type: ResponseType, status_code: Option<String>) -> Self {
            let http_status: Result<String, String> = Ok(status_code.unwrap_or_else(||String::from("200")));
            Self::new( val, r_type, http_status)
    }


    pub fn err(val: String, r_type: ResponseType, status_code: Option<String>) -> Self{
            let http_status: Result<String, String> = Ok(status_code.unwrap_or_else(||String::from("403")));
            Self::new( val, r_type, http_status)
    }

    pub fn send(&self) -> Vec<String> {
            match self.r_type {
                    ResponseType::Raw => vec![self.val.clone(), self.http_status.clone().unwrap()],
                    ResponseType::HTML => vec![Self::send_html_response(&self.val), self.http_status.clone().unwrap()]
            }
        }

        pub fn send_html_response(fname :&str) -> String {
            match fs::read_to_string(fname) {
                Ok(content) => content, 
                Err(e) => panic!("FILE NOT FOUND")
            }
        }
}