mod request;
mod response;
mod route;

use std::io;
use regex::Regex;
use route::{HttpMethod, Route};
use request::{Request};
use std::net::{SocketAddr, TcpListener, TcpStream};
use socket2::{Socket, Domain, Type};
use std::collections::HashMap;

type Handler = fn(request: Request) -> response::Response;

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

        let socket = Socket::new(Domain::ipv4(), Type::stream(), None).expect("Failed to bind to port.");
        socket.bind(&address.parse::<SocketAddr>().unwrap().into()).unwrap();
        socket.set_nonblocking(true).expect("Failed to set non-blocking.");
        self.listen(socket.into_tcp_listener());

        println!("Done!");
    }

    async fn listen(self, listener: TcpListener) {
        for stream in listener.incoming() {
            match stream {
                Ok(s) => {
                    self.handle_request(&stream.unwrap());
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => panic!("IO Error: {}", e)
            }
        }
    }

    async fn handle_request(self, stream: &TcpStream) {
        let request = Request::new(stream);
        let uri = &request.uri;
        for (route, handler) in self.routes {
            let regex = Regex::new(&route.regex).unwrap();
            if regex.is_match(&uri) {
                handler(request);
                break;
            }
        }
    }
}
