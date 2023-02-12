use std::fs;

use super::http::{Method, Request, Response, StatusCode};
use super::server::Handler;

pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    pub fn read_file(&self, file_name: &str) -> Option<String> {
        let file_path = format!("{}/{}", self.public_path, file_name); 
        fs::read_to_string(file_path).ok()
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, req: &Request) -> Response {
        match req.method() {
            Method::GET => match req.path() {
                "/" => Response::new(
                    StatusCode::Ok,
                    self.read_file("index.html"),
                ),
                path => match self.read_file(path) {
                    Some(content) => Response::new(StatusCode::Ok, Some(content)),
                    None => Response::new(StatusCode::NotFound, None),
                },
            },
            _ => Response::new(StatusCode::NotImplemented, None),
        }
    }
}
