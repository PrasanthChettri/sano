use tiny_http::{Server, Request, Response, Header, StatusCode};
use serde_json::Value;
use serde_urlencoded;

fn handle_request(request: Request) {
    let content_type = request.headers().iter().find(|h| h.field == "Content-Type");
    match content_type {
        Some(header) if header.value.contains("application/json") => {
            // Handle JSON request
            handle_json_request(request);
        }
        Some(header) if header.value.contains("application/x-www-form-urlencoded") => {
            // Handle form request
            handle_form_request(request);
        }
        _ => {
            // Handle other content types or no content type
            let response = Response::from_string("Unsupported Content-Type")
                .with_status_code(StatusCode::from(415));
            request.respond(response).expect("Failed to send response");
        }
    }
}

fn handle_json_request(request: Request) {
    let mut buffer = Vec::new();
    request
        .as_reader()
        .read_to_end(&mut buffer)
        .expect("Failed to read request body");

    let body = String::from_utf8_lossy(&buffer);

    // Parse JSON
    match serde_json::from_str::<Value>(&body) {
        Ok(json) => {
            // Handle the parsed JSON
            println!("Received JSON: {:?}", json);

            // Send response
            let response = Response::from_string("JSON received successfully");
            request.respond(response).expect("Failed to send response");
        }
        Err(e) => {
            // Handle JSON parsing error
            eprintln!("Error parsing JSON: {:?}", e);

            // Send error response
            let response = Response::from_string("Error parsing JSON")
                .with_status_code(StatusCode::from(400));
            request.respond(response).expect("Failed to send response");
        }
    }
}

fn handle_form_request(request: Request) {
    let mut buffer = Vec::new();
    request
        .as_reader()
        .read_to_end(&mut buffer)
        .expect("Failed to read request body");

    let body = String::from_utf8_lossy(&buffer);

    // Parse form data
    match serde_urlencoded::from_str::<HashMap<String, String>>(&body) {
        Ok(form_data) => {
            // Handle the parsed form data
            println!("Received form data: {:?}", form_data);

            // Send response
            let response = Response::from_string("Form data received successfully");
            request.respond(response).expect("Failed to send response");
        }
        Err(e) => {
            // Handle form data parsing error
            eprintln!("Error parsing form data: {:?}", e);

            // Send error response
            let response = Response::from_string("Error parsing form data")
                .with_status_code(StatusCode::from(400));
            request.respond(response).expect("Failed to send response");
        }
    }
}