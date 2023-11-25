use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

pub struct Server {
    listener : TcpListener,
}

impl Server {
    pub fn new(ipaddr : &String , port: u16) -> Self{
        let address = format!("{}:{}", ipaddr, port);
        let listener = TcpListener::bind(address).expect("Failed to bind to the specified address");
        Self { listener }
    }

    pub fn run_server<F> (&self, f: F) 
    where F: Fn(Vec<String>) -> Vec<String>{
        for stream in self.listener.incoming() {
            let stream = stream.unwrap();
            let request = Self::get_raw_request(&stream);
            //visitor pattern ? 
            let response = f(request) ;
            Self::send_raw_response(stream, &response[0], &response[1]) ;
        }
    }

    pub fn get_raw_request(stream: &TcpStream) -> Vec<String> {
        let buf_reader = BufReader::new(stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
        return http_request; 
    }

    pub fn send_raw_response(mut stream: TcpStream, response :&String, status_code: &str){
        let status_line = format!("HTTP/1.1 {} OK", status_code);
        let length = response.len();
        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{response}");
        stream.write_all(response.as_bytes()).unwrap()
    }
}

