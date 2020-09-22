mod request;
mod response;
mod route;

use route::{HttpMethod, Route};
use request::{Request};
use std::net::{SocketAddr, TcpListener};
use socket2::{Socket, Domain, Type};
use std::collections::HashMap;

type Handler = fn(request: request::Request) -> response::Response;

struct HttpServer {
    port: i16,
    routes: HashMap<Route, Handler>
}

impl HttpServer {
    pub fn bind(port: i16) -> HttpServer {
        HttpServer {
            port: port,
            routes: HashMap::new()
        }
    }

    pub fn get(self, endpoint: &String, handler: Handler) -> HttpServer {
        self.route(endpoint, HttpMethod::Get, handler)
    }

    pub fn post(self, endpoint: &String, handler: Handler) -> HttpServer {
        self.route(endpoint, HttpMethod::Post, handler)
    }

    pub fn route(mut self, endpoint: &String, method: HttpMethod, handler: Handler) -> HttpServer {
        let route = Route::new(endpoint, method);
        if self.routes.contains_key(&route) {
            panic!("Unable to define duplicate route {} with specified HTTP method.", endpoint);
        }
        self.routes.insert(route, handler);
        self
    }

    pub fn launch(self) {
        let address = format!("127.0.0.1:{}", self.port);
        println!("Launching at {}...", address);

        let socket = Socket::new(Domain::ipv4(), Type::stream(), None).unwrap();
        socket.bind(&address.parse::<SocketAddr>().unwrap().into()).unwrap();
        self.listen(socket.into_tcp_listener());

        println!("Done!");
    }

    async fn listen(self, listener: TcpListener) {
        loop {
            let result = listener.accept();
        }
    }

    async fn handle_request(self, request: Request) {

    }
}
