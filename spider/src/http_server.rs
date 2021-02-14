use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::Write;
use std::str;

use crate::request::Request;
use crate::response::Response;
use crate::http_method::HttpMethod;

pub trait HttpHandler {
    fn do_get(&self, _request: Request) -> Response {
        return self.default_action(_request);
    }

    fn do_head(&self, _request: Request) -> Response {
        return self.default_action(_request);
    }

    fn do_post(&self, _request: Request) -> Response {
        return self.default_action(_request);
    }

    fn do_put(&self, _request: Request) -> Response {
        return self.default_action(_request);
    }

    fn do_delete(&self, _request: Request) -> Response {
        return self.default_action(_request);
    }

    fn do_connect(&self, _request: Request) -> Response {
        return self.default_action(_request);
    }

    fn do_options(&self, _request: Request) -> Response {
        return self.default_action(_request);
    }

    fn do_trace(&self, _request: Request) -> Response {
        return self.default_action(_request);
    }

    fn do_patch(&self, _request: Request) -> Response {
        return self.default_action(_request);
    }

    fn default_action(&self, _request: Request) -> Response {
        let msg = String::from("<h1>Method not allowed</h1>");
        return Response::html(msg, 405);
    }
}

pub struct HttpServer<T: HttpHandler> {
    host: String,
    port: u16,
    socket: TcpListener,
    handler: T
}

impl<T> HttpServer<T> where T: HttpHandler {
    // TODO(andrew): add explanations for errors?
    pub fn new(host: &str, port: u16, handler: T) -> Result<HttpServer<T>, &str> {
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
                return Ok(server);
            },
            Err(_) => return Err("Couldn't start server")
        };
    }

    pub fn serve_forever(&self) -> Result<(), &str> {
        for stream in self.socket.incoming() {
            match stream {
                Ok(s) => {
                    // TODO(andrew): replace println with logging.
                    println!("Got connection!");
                    match self.handle_client(&s) {
                        Ok(_) => (),
                        Err(msg) => return Err(msg)
                    };
                    match s.shutdown(Shutdown::Both) {
                        Ok(_) => println!("Closed connection"),
                        Err(_) => return Err("Couldn't close client socket")
                    };
                },
                Err(_) => break
            };
        }
        return Ok(());
    }

    fn handle_client(&self, mut stream: &TcpStream) -> Result<(), &str> {
        let mut buf: [u8; 1024] = [0; 1024];
        // TODO(andrew): read all body, not first 1024 bytes.
        stream.peek(&mut buf).expect("Couldn't read from socket");

        let request = Request::from(&buf);
        let request = match request {
            Some(r) => r,
            None => return Err("Request parsed with errors")
        };

        let response = match request.method {
            HttpMethod::GET     => self.handler.do_get(request),
            HttpMethod::HEAD    => self.handler.do_head(request),
            HttpMethod::POST    => self.handler.do_post(request),
            HttpMethod::PUT     => self.handler.do_put(request),
            HttpMethod::DELETE  => self.handler.do_delete(request),
            HttpMethod::CONNECT => self.handler.do_connect(request),
            HttpMethod::OPTIONS => self.handler.do_options(request),
            HttpMethod::TRACE   => self.handler.do_trace(request),
            HttpMethod::PATCH   => self.handler.do_patch(request),
        };
        let response = response.format();
        match stream.write(&response) {
            Ok(_) => return Ok(()),
            Err(_) => return Err("Couldn't write to client socket")
        };
    }
}
