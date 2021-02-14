use std::collections::HashMap;
use std::fmt;

use crate::http_status::get_status_text;

pub struct Response {
    code: u16,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

// TODO(andrew): add more constructors for different content types.
impl Response {
    pub fn html(html: String, status_code: u16) -> Response {
        let mut headers = HashMap::new();
        headers.insert( String::from("Content-Type")
                      , String::from("text/html") );

        return Response {
            code: status_code,
            headers: headers,
            body: html.as_bytes().to_vec(),
        };
    }

    pub fn format<'a>(&self) -> Vec<u8> {
        let status_text = match get_status_text(self.code) {
            Some(text) => text,
            None => String::from("UNDEFINED")
        };

        let mut data = Vec::new();

        let first_line = format!("HTTP/1.1 {} {}", self.code, status_text);
        let headers = format_headers(&self.headers);
        let head = format!("{}\r\n{}\r\n", first_line, headers);

        data.extend_from_slice(head.as_bytes());
        data.extend_from_slice(&self.body);
        return data;
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "Response({})", self.code);
    }
}

fn format_headers(headers: &HashMap<String, String>) -> String {
    let mut result = String::new();
    for (key, value) in headers.iter() {
        let line = format!("{}: {}\r\n", key, value);
        result.push_str(&line);
    }
    return result;
}
