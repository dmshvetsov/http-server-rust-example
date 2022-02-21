use crate::{Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    // Associative functions

    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    // Methods

    pub fn run(self) {
        let listener = TcpListener::bind(&self.addr).unwrap();
        println!("HTTP/1.1 server is listening on {}", self.addr);

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buf = [0; 1024];
                    match stream.read(&mut buf) {
                        Ok(_) => {
                            // println!("{}", String::from_utf8_lossy(&buf));
                            match Request::try_from(&buf[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                    let res = Response::new(
                                        StatusCode::Ok,
                                        Some("<h1>Rust Server Up and Running</h1>".to_string()),
                                    );
                                    res.send(&mut stream);
                                }
                                Err(e) => println!("Failed to parse request: {}", e),
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
