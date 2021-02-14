use spider::http_server::{HttpHandler, HttpServer};
use spider::request::Request;
use spider::response::Response;

struct MyHandler {}

impl HttpHandler for MyHandler {
    fn do_get(&self, _request: Request) -> Response {
        return Response::html(String::from("hey"), 200);
    }

    fn do_post(&self, _request: Request) -> Response {
        return Response::html(String::from("hey"), 200);
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

    let result = match server {
        Ok(ref serv) => serv.serve_forever(),
        Err(msg) => Err(msg)
    };
    match result {
        Ok(_) => println!("Server exited succesfully"),
        Err(msg) => println!("{}", msg)
    };
}
