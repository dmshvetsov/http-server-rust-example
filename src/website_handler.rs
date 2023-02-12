use super::http::{Method, Request, Response, StatusCode};
use super::server::Handler;

pub struct WebsiteHandler;

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, req: &Request) -> Response {
        match req.method() {
            Method::GET => match req.path() {
                "/" => Response::new(
                    StatusCode::Ok,
                    Some("<h1>Rust Server Up and Running</h1>".to_string()),
                ),
                _ => Response::new(StatusCode::NotFound, None),
            },
            _ => Response::new(StatusCode::NotImplemented, None),
        }
    }
}
