use crate::http::ParseError;
use crate::{Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub trait Handler {
    fn handle_request(&mut self, req: &Request) -> Response;

    fn handle_parse_error(&mut self, err: &ParseError) -> Response {
        print!("Failed to parse HTTP request: {}", err);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    // Associative functions

    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    // Methods

    pub fn run(self, mut handler: impl Handler) {
        let listener = TcpListener::bind(&self.addr).unwrap();
        println!("HTTP/1.1 server is listening on {}", self.addr);

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buf = [0; 1024];
                    match stream.read(&mut buf) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buf));

                            let res = match Request::try_from(&buf[..]) {
                                Ok(req) => {
                                    handler.handle_request(&req)
                                }
                                Err(err) => {
                                    handler.handle_parse_error(&err)
                                }
                            };
                            if let Err(e) = res.send(&mut stream) {
                                println!("Failed to send response: { }", e);
                            }
                        }
                        Err(e) => println!("Failed to read request: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}
