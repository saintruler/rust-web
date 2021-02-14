use std::collections::HashMap;
use std::fmt;

pub struct Response {
    status: i32,
    headers: HashMap<String, String>,
    body: String
}

impl Response {
    pub fn new(body: &str) -> Response {
        let headers: HashMap<String, String> = HashMap::new();
        return Response {
            status: 200,
            headers: headers,
            body: String::from(body)
        };
    }

    pub fn format<'a>(&self) -> &'a [u8] {
        let s = "HTTP/1.1 200 OK\r\nConnection: keep-alive\r\nContent-Type: text/html\r\n\r\n<i>Hello</i>";
        return s.as_bytes();

        // let buf: &[u8];
        // buf = &[0; 1024];
//
        // return buf;
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "Response({})", self.status);
    }
}
