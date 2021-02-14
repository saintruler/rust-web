use std::net::{TcpListener, TcpStream};
use std::str;
use std::collections::HashMap;
use std::fmt;

enum HttpMethod {
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

struct Request {
    method: HttpMethod,
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

    pub fn print(&self) {
        println!("Request({}, {})", self.method, self.path);
    }
}

struct Response {
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
}

fn do_get(request: Request) -> Response {
    return Response::new("hey");
}

fn do_post(request: Request) -> Response {
    return Response::new("hey");
}

// TODO(andrew): Add some error handling.
fn parse_request(data: &str) -> Option<Request> {
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

fn format_response<'a>(response: Response) -> &'a [u8] {
    let buf: &[u8];
    buf = &[0; 1024];


    return buf;
}

fn handle_client(stream: TcpStream) {
    let mut buf: [u8; 1024] = [0; 1024];
    stream.peek(&mut buf).expect("Couldn't read from socket");

    let s = match str::from_utf8(&buf) {
        Ok(v) => v,
        Err(_e) => panic!("Couldn't convert u8 to character")
    };

    let request = parse_request(&s);
    // TODO(andrew): remove panic!.
    let request = match request {
        Some(r) => r,
        None => panic!("Request parsed with errors")
    };

    let response = match request.method {
        HttpMethod::GET  => do_get(request),
        HttpMethod::POST => do_post(request)
    };
    let response = format_response(response);
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("localhost:3000")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }

    Ok(())
}
