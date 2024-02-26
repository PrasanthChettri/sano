use std::collections::HashMap;

use crate::response::Body as SaonResponseBody ;
use crate::{
    Response as SaonResponse,
    request::{Request as sanoRequest, Method as SanoMethod, Body},
    sano
};

use tiny_http::{
    Server as TinyHttpServer,
    Request as TinyHttpRequest,
    Method as TinyHttpMethod,
    Response as TinyHttpResponse,
    StatusCode, HeaderField
};
use url::Url;


pub fn getContentType(request: &TinyHttpRequest, string_body: String) -> Body {
    for header in request.headers() {
        if header.field.equiv("Content-Type"){
            let ctype = header.value.to_string() ;
            if ctype == "application/json" {
                return Body::Json(string_body);
            }
            else if ctype.starts_with("multipart/form-data") {
                return Body::Form(string_body);
            }
            else if ctype.starts_with("application/x-www-form-urlencoded") {
                return Body::FormUrlEncoded(string_body);
            }
            else if ctype.starts_with("text/plain") {
                return Body::Text(string_body);
            }
            else {
                todo!();
            }
        }
    }
    Body::Nil
}
pub fn serialize_from_tiny_http(tinyrequest: &mut TinyHttpRequest) -> Result<sanoRequest, Box<dyn std::error::Error>> {
    let mut body = Vec::new();
    tinyrequest.as_reader().read_to_end(&mut body)?;

    let body = String::from_utf8_lossy(&body).into_owned();
    let body = getContentType(&tinyrequest, body);

    let complete_url = format!("http://localhost:7879{}", tinyrequest.url());
    let binding = Url::parse(complete_url.as_ref())?;

    let query_params: HashMap<_, _> = binding.query_pairs().into_owned().collect() ;


    let partial_url = binding.path().to_string() ;
    let method = from_tiny_http_method(tinyrequest.method());

    Ok(
        sanoRequest::new(partial_url, query_params, method, body)
    )
}

pub fn respond_with_tiny_http(sano_response: SaonResponse) -> TinyHttpResponse<std::io::Cursor<Vec<u8>>> {
        let status = sano_response.http_status ;
        let response = match &sano_response.body {
            SaonResponseBody::Raw(e) => e,
            _ => "E"
        };
        TinyHttpResponse::from_string(response)
                    .with_status_code(status)
}

// Static function for conversion
pub fn from_tiny_http_method(tiny_http_method: &TinyHttpMethod) -> SanoMethod {
    match tiny_http_method {
        TinyHttpMethod::Get => SanoMethod::GET,
        TinyHttpMethod::Post => SanoMethod::POST,
        TinyHttpMethod::Put => SanoMethod::PUT,
        TinyHttpMethod::Delete => SanoMethod::DELETE,
        TinyHttpMethod::Patch => SanoMethod::PATCH,
        TinyHttpMethod::Options => SanoMethod::OPTIONS,
        TinyHttpMethod::Head => SanoMethod::HEAD,
        TinyHttpMethod::Trace => SanoMethod::TRACE,
        TinyHttpMethod::Connect => SanoMethod::CONNECT,
        _ =>SanoMethod::NONSTANDARD,
    }
}
