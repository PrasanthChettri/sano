use std::fs ;

pub trait ResponseSerializable {
    fn serialze(self) -> Response;
}

#[derive(Debug, Clone)]
pub enum Body {
    HTML(String), 
    Raw(String),
    Nil
}

impl Default for Body {
    fn default() -> Self {
        Body::Nil
    }
}

#[derive(Debug)]
pub struct Response{
    pub body: Body,
    pub http_status: u16
}

impl Default for Response{
    fn default() -> Self {
        Self {
            body: Body::default(),
            http_status: 204
        }
    }
}

#[macro_export]
macro_rules! impl_response_serializable_for_to_string {
    ($($type:ty),*) => {
        $(
            impl ResponseSerializable for $type {
                fn serialze(self) -> Response {
                    Response::new(Body::Raw(self.to_string().into()), 200)
                }
            }
        )*
    };
}

impl_response_serializable_for_to_string! { &str, f32, i8, i16, i32, f64, u64, u32, u8, u16, i64, i128 }

impl ResponseSerializable for String {
    fn serialze(self) -> Response {
        Response::new(Body::Raw(self), 200)
    }
}

impl ResponseSerializable for Response {
    fn serialze(self) -> Response {
        self
    }
}


impl Response {
    pub fn new(body: Body, http_status: u16) -> Self{
        Self { body , http_status }
    }

    pub fn ok(body: Body) -> Self { Self::new(body, 200) }
    pub fn err(body: Body) -> Self { Self::new(body, 400) }
}

pub struct ResponseBldr {
    response: Response,
}

impl ResponseSerializable for ResponseBldr {
    fn serialze(self) -> Response {
        self.give()
    }
}

impl ResponseBldr {
    pub fn new() -> Self { ResponseBldr { response:  Response::default() } }

    pub fn body(mut self, body: Body) -> ResponseBldr {
        self.response.body = body;
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
