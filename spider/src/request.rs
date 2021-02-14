use std::collections::HashMap;
use std::fmt;

use crate::http_method::HttpMethod;

pub struct Request {
    pub method: HttpMethod,
    path: String,
    http_version: String,
    headers: HashMap<String, String>,
    // body: &'a [u8]
}

impl Request {
    pub fn new( method: HttpMethod
              , path: String
              , http_version: String
              , headers: HashMap<String, String>) -> Request {
        return Request {
            method,
            path,
            http_version,
            headers
        };
    }

    // TODO(andrew): accept &[u8] instead of &str and add parsing of body.
    // TODO(andrew): add some error handling.
    pub fn from(data: &str) -> Option<Request> {
        let lines = data.split("\r\n").collect::<Vec<&str>>();

        let mut headers: HashMap<String, String> = HashMap::new();
        for line in &lines[1..] {
            let line = line.split(": ").collect::<Vec<&str>>();
            headers.insert(String::from(line[0]), line[1..].join(" "));
        }

        let first_line = lines[0].split(" ").collect::<Vec<&str>>();

        let method = String::from(first_line[0]);
        match HttpMethod::parse(method) {
            Some(method) =>
                return Some(Request::new(
                    method,
                    String::from(first_line[1]),
                    String::from(first_line[2]),
                    headers )),
            None => return None
        };
    }
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "Request({}, {})", self.method, self.path);
    }
}
