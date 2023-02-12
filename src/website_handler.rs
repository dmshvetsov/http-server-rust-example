use super::server::Handler;
use super::http::{Request, Response, StatusCode};

pub struct WebsiteHandler;

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, req: &Request) -> Response {
        Response::new(StatusCode::Ok, Some("<h1>Rust Server Up and Running</h1>".to_string()))
    }
}
