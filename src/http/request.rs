use std::str;
use std::str::Utf8Error;
use std::str::FromStr;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

// Request

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

impl <'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }
}

// Method

#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    OPTION,
    HEAD,
    CONNECT,
    TRACE,
}

impl FromStr for Method {
    type Err = UnknownMethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::GET),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "PATCH" => Ok(Self::PATCH),
            "DELETE" => Ok(Self::DELETE),
            "OPTION" => Ok(Self::OPTION),
            "HEAD" => Ok(Self::HEAD),
            "CONNECT" => Ok(Self::CONNECT),
            "TRACE" => Ok(Self::TRACE),
            _ => Err(UnknownMethodError),
        }
    }
}

pub struct UnknownMethodError;

// QueryString

#[derive(Debug)]
pub struct QueryString<'buf> {
    raw: &'buf str,
    params: HashMap<&'buf str, QueryParam<'buf>>,
}

#[derive(Debug)]
pub enum QueryParam<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    fn get(&self, key: &str) -> Option<&QueryParam> {
        self.params.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(raw_query_string: &'buf str) -> Self {
        let mut params = HashMap::new();
        for key_val in raw_query_string.split('&') {
            let mut key = key_val;
            let mut val = "";
            if let Some(idx) = key_val.find('=') {
                key = &key_val[..idx];
                val = &key_val[idx + 1..];
            }

            params.entry(key).and_modify(|existing_val: &mut QueryParam| match existing_val {
                QueryParam::Single(prev_val) => {
                    *existing_val = QueryParam::Multiple(vec![prev_val, val]);
                },
                QueryParam::Multiple(existing_vec) => existing_vec.push(val),
            }).or_insert(QueryParam::Single(val));
        };
        QueryString { raw: raw_query_string, params }
    }
}

// Helpers

fn get_next_word(raw_request: &str) -> Option<(&str, &str)> {
    for (idx, c) in raw_request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some(
                (&raw_request[..idx], &raw_request[idx + 1..])
            );
        }
    }

    None
}

// ...

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        let raw_request = str::from_utf8(buf)?;

        let (method, raw_request) = get_next_word(raw_request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, raw_request) = get_next_word(raw_request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(raw_request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            // only supports HTTP 1.1
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;
        if let Some(idx) = path.find('?') {
            query_string = Some(QueryString::from(&path[&idx + 1..]));
            path = &path[..idx];
        }

        Ok(Self {
            method,
            path,
            query_string,
        })
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl From<UnknownMethodError> for ParseError {
    fn from(_: UnknownMethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidRequest
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}
