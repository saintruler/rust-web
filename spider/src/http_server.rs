use std::net::{TcpListener, TcpStream};
use std::io::Write;
use std::str;

use crate::request::Request;
use crate::response::Response;
use crate::http_method::HttpMethod;

pub trait HttpHandler {
    fn do_get(&self, request: Request) -> Response;
    fn do_post(&self, request: Request) -> Response;
}

pub struct HttpServer<T: HttpHandler> {
    host: String,
    port: u16,
    socket: TcpListener,
    handler: T
}

impl<T> HttpServer<T> where T: HttpHandler {
    // TODO(andrew): Add more verbose error handling.
    pub fn new(host: &str, port: u16, handler: T) -> Option<HttpServer<T>> {
        let addr = format!("{}:{}", host, port);
        let sock = TcpListener::bind(addr);
        match sock {
            Ok(s) => {
                let server = HttpServer {
                    host: String::from(host),
                    port: port,
                    socket: s,
                    handler: handler,
                };
                return Some(server);
            },
            Err(_) => return None
        }
    }

    pub fn serve_forever(&self) {
        for stream in self.socket.incoming() {
            match stream {
                Ok(s) => {
                    self.handle_client(s);
                },
                Err(_) => break
            };
        }
    }

    fn handle_client(&self, mut stream: TcpStream) {
        let mut buf: [u8; 1024] = [0; 1024];
        stream.peek(&mut buf).expect("Couldn't read from socket");

        // TODO(andrew): remove panic.
        let s = match str::from_utf8(&buf) {
            Ok(v) => v,
            Err(_) => panic!("Couldn't convert u8 to character")
        };

        let request = Request::from(&s);
        // TODO(andrew): remove panic.
        let request = match request {
            Some(r) => r,
            None => panic!("Request parsed with errors")
        };

        let response = match request.method {
            HttpMethod::GET  => self.handler.do_get(request),
            HttpMethod::POST => self.handler.do_post(request)
        };
        let response = response.format();
        stream.write(response);
    }
}
