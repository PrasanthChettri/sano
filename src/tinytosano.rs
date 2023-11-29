use std::collections::HashMap;

use crate::{
    Response as SaonResponse,
    request::{Request as sanoRequest, Method},
    sano
};

use tiny_http::{
    Server as TinyHttpServer,
    Request as TinyHttpRequest,
    Response as TinyHttpResponse,
    StatusCode, HeaderField
};
use url::Url;

pub fn serialize_from_tiny_http(tinyrequest: &mut TinyHttpRequest) -> sanoRequest {
        let body = {
            let mut body = Vec::new();
            tinyrequest.as_reader().read_to_end(&mut body).unwrap();
            String::from_utf8_lossy(&body).into_owned()
        };
        let complete_url = format!("http://localhost:7879{}", tinyrequest.url());
        let binding =  Url::parse(&complete_url).unwrap();
        let query_params: HashMap<_, _> = binding.query_pairs().into_owned().collect();
        let partial_url = String::from(binding.path());
        let method = Method::GET;//tinyrequest.method().as_str());
        let request = sanoRequest::new(partial_url, Some(query_params), method, Some(body));
        return request;
}
pub fn respond_with_tiny_http(sano_response: SaonResponse) -> TinyHttpResponse<std::io::Cursor<Vec<u8>>> {
        let status = (sano_response.http_status).to_string();
        let response = &sano_response.send_response_body();
        TinyHttpResponse::from_string(response)
                    .with_status_code(status.parse::<u16>().unwrap())
}