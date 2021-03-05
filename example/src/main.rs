use toy_http::request::Request;
use toy_http::response::Response;
use toy_http::HttpServer;

fn index(request: &Request) -> Response {
    Response::new("Hello world")
}

fn api_param(request: &Request) -> Response {
    let param: i32 = request.get_param("param");
    let body = format!("Received int {}", param);
    Response::new(&body)
}

fn main() {
    let server = HttpServer::bind(5000)
        .get("/", index)
        .get("/api/<param:int>", api_param);
    server.launch();
}
