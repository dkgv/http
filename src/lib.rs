mod request;
mod response;
mod route;

use regex::Regex;
use request::Request;
use response::Response;
use route::{HttpMethod, Route};
use socket2::{Domain, Socket, Type};
use std::collections::HashMap;
use std::io;
use std::net::{SocketAddr, TcpListener, TcpStream};

type Handler = fn(request: Request) -> response::Response;

struct HttpServer {
    port: i16,
    routes: HashMap<Route, Handler>,
}

impl HttpServer {
    pub fn bind(port: i16) -> HttpServer {
        HttpServer {
            port: port,
            routes: HashMap::new(),
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
            panic!(
                "Unable to define duplicate route {} with specified HTTP method.",
                endpoint
            );
        }
        self.routes.insert(route, handler);
        self
    }

    pub fn launch(self) {
        let address = format!("127.0.0.1:{}", self.port);
        println!("Launching at {}...", address);

        let socket =
            Socket::new(Domain::ipv4(), Type::stream(), None).expect("Failed to bind to port.");
        socket
            .bind(&address.parse::<SocketAddr>().unwrap().into())
            .unwrap();
        socket
            .set_nonblocking(true)
            .expect("Failed to set non-blocking.");
        self.listen(socket.into_tcp_listener());
    }

    async fn listen(self, listener: TcpListener) {
        let routes = self.routes;
        let mut incoming = listener.incoming();

        while let Some(tcp_stream) = incoming.next() {
            match tcp_stream {
                Ok(s) => {
                    handle_request(&s, &routes);
                }
                Err(e) => {
                    if e.kind() == io::ErrorKind::WouldBlock {
                        continue;
                    }
                    panic!("Error");
                }
            };
        }
    }
}

async fn handle_request(stream: &TcpStream, routes: &HashMap<Route, Handler>) {
    let request = Request::new("TODO GET /endpoint HTTP 1.1".to_string());
    let uri = &request.uri;
    for (route, handler) in routes {
        let regex = Regex::new(&route.regex).unwrap();
        if regex.is_match(&uri) {
            dispatch_response(stream, handler(request)).await;
            break;
        }
    }
}

async fn dispatch_response(stream: &TcpStream, response: Response) {
    let mut body = response.body;
    // TODO write body
}
