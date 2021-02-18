use std::collections::HashMap;
use std::fmt;

use crate::http_method::HttpMethod;

pub struct Request {
    pub method: HttpMethod,
    pub path: String,
    pub http_version: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Request {
    pub fn new( method: HttpMethod
              , path: String
              , http_version: String
              , headers: HashMap<String, String>
              , body: Vec<u8> ) -> Request {
        return Request {
            method,
            path,
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

        let mut body = data[idx..].to_vec();

        let mut headers: HashMap<String, String> = HashMap::new();
        if lines.len() > 0 {
            for line in &lines[1..] {
                let line = line
                    .trim_end_matches("\r\n")
                    .split(": ")
                    .collect::<Vec<&str>>();
                headers.insert(String::from(line[0]), line[1..].join(" "));
            }
        }

        let request_line = lines[0]
            .trim_end_matches("\r\n")
            .split(" ")
            .collect::<Vec<&str>>();

        let (path, params) = split_path(String::from(request_line[1]));

        let method = String::from(request_line[0]);
        match HttpMethod::parse(method) {
            Some(method) => {
                if method == HttpMethod::GET {
                    if let Some(p) = params {
                        body = p.as_bytes().to_vec();
                    }
                }
                return Some(Request::new(
                    method,
                    path,
                    String::from(request_line[2]),
                    headers,
                    body ));
            }
            None => return None
        };
    }
}

fn split_path(resource: String) -> (String, Option<String>) {
    let mut idx = None;
    for (i, c) in resource.chars().enumerate() {
        if c == '?' {
            idx = Some(i);
            break;
        }
    }
    match idx {
        Some(n) => {
            let path = &resource[..n];
            let params = &resource[n + 1..];
            return (path.to_string(), Some(params.to_string()));
        },
        None => return (resource, None)
    };
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "Request({}, {})", self.method, self.path);
    }
}
