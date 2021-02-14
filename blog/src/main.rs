use spider::http_server::{HttpHandler, HttpServer};
use spider::request::Request;
use spider::response::Response;

struct MyHandler {}

impl HttpHandler for MyHandler {
    fn do_get(&self, _request: Request) -> Response {
        return Response::new("hey");
    }

    fn do_post(&self, _request: Request) -> Response {
        return Response::new("hey");
    }
}

impl MyHandler {
    pub fn new() -> MyHandler {
        return MyHandler {};
    }
}

// TODO(andrew): create logging package.
fn main() {
    let handler = MyHandler::new();
    let server = HttpServer::new("localhost", 3000, handler);
    match server {
        Some(serv) => serv.serve_forever(),
        None => println!("Couldn't start server.")
    }
}
