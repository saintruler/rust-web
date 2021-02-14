use std::collections::HashMap;
use std::fmt;

use crate::http_method::HttpMethod;

pub struct Request {
    pub method: HttpMethod,
    resource: String,
    http_version: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl Request {
    pub fn new( method: HttpMethod
              , resource: String
              , http_version: String
              , headers: HashMap<String, String>
              , body: Vec<u8> ) -> Request {
        return Request {
            method,
            resource,
            http_version,
            headers,
            body,
        };
    }

    // TODO(andrew): add some error handling.
    pub fn from(data: &[u8]) -> Option<Request> {
        let mut lines: Vec<String> = Vec::new();
        let mut line = String::new();
        let mut idx = 0;
        let mut char_count = 0;

        // Parsing headers until first empty line (char_count == 0).
        for c in data {
            let c = *c as char;
            line.push(c);
            idx += 1;

            if c == '\n' {
                lines.push(line);
                if char_count == 0 { break; }
                else { line = String::new(); }
            }
            if c != '\r' && c != '\n' {
                char_count += 1;
            }
        }

        let body = data[idx..].to_vec();

        let mut headers: HashMap<String, String> = HashMap::new();
        for line in &lines[1..] {
            let line = line
                .trim_end_matches("\r\n")
                .split(": ")
                .collect::<Vec<&str>>();
            headers.insert(String::from(line[0]), line[1..].join(" "));
        }

        let request_line = lines[0]
            .trim_end_matches("\r\n")
            .split(" ")
            .collect::<Vec<&str>>();

        let method = String::from(request_line[0]);
        match HttpMethod::parse(method) {
            Some(method) =>
                return Some(Request::new(
                    method,
                    // TODO(andrew): add parsing of resource line.
                    String::from(request_line[1]),
                    String::from(request_line[2]),
                    headers,
                    body )),
            None => return None
        };
    }
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "Request({}, {})", self.method, self.resource);
    }
}
