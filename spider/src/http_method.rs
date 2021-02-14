use std::fmt;

pub enum HttpMethod {
    GET, POST
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HttpMethod::GET => write!(f, "GET"),
            HttpMethod::POST => write!(f, "POST")
        }
    }
}

impl HttpMethod {
    pub fn parse(s: String) -> Option<HttpMethod> {
        if s == "GET" {
            return Some(HttpMethod::GET);
        }
        else if s == "POST" {
            return Some(HttpMethod::POST);
        }
        else {
            return None;
        }
    }
}
