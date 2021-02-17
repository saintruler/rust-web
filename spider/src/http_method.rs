use std::fmt;

#[derive(PartialEq)]
pub enum HttpMethod {
    GET, HEAD, POST, PUT, DELETE, CONNECT, OPTIONS, TRACE, PATCH
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HttpMethod::GET     => write!(f, "GET"),
            HttpMethod::HEAD    => write!(f, "HEAD"),
            HttpMethod::POST    => write!(f, "POST"),
            HttpMethod::PUT     => write!(f, "PUT"),
            HttpMethod::DELETE  => write!(f, "DELETE"),
            HttpMethod::CONNECT => write!(f, "CONNECT"),
            HttpMethod::OPTIONS => write!(f, "OPTIONS"),
            HttpMethod::TRACE   => write!(f, "TRACE"),
            HttpMethod::PATCH   => write!(f, "PATCH"),
        }
    }
}

impl HttpMethod {
    pub fn parse(s: String) -> Option<HttpMethod> {
        match &*s {
            "GET"     => Some(HttpMethod::GET),
            "HEAD"    => Some(HttpMethod::HEAD),
            "POST"    => Some(HttpMethod::POST),
            "PUT"     => Some(HttpMethod::PUT),
            "DELETE"  => Some(HttpMethod::DELETE),
            "CONNECT" => Some(HttpMethod::CONNECT),
            "OPTIONS" => Some(HttpMethod::OPTIONS),
            "TRACE"   => Some(HttpMethod::TRACE),
            "PATCH"   => Some(HttpMethod::PATCH),
            _ => None
        }
    }
}
