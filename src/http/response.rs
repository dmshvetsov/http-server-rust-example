use std::fmt::{Display, Formatter, Result as FmtResult};
use std::net::TcpStream;
use std::io::{Write, Result as IoResult};

#[derive(Clone, Copy, Debug)]
pub enum StatusCode {
    Ok = 200,
    Created = 201,
    PermanentlyMoved = 301,
    NotModified = 304,
    BadRequest = 400,
    Unauthorized = 401,
    Forbiden = 403,
    NotFound = 404,
    Conflict = 409,
    InternalServerError = 500,
}

impl StatusCode {
    fn reason_phrase(&self) -> &str {
        match self {
            Self::Ok => "Ok",
            Self::Created => "Created",
            Self::PermanentlyMoved => "Permanently moved",
            Self::NotModified => "Not modified",
            Self::BadRequest => "Bad request",
            Self::Unauthorized => "Unauthorized",
            Self::Forbiden => "Forbiden",
            Self::NotFound => "Not found",
            Self::Conflict => "Conflict",
            Self::InternalServerError => "Internal server error",
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", *self as u16)
    }
}

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }

    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            match &self.body {
                Some(content) => content,
                None => "",
            },
        )
    }
}
