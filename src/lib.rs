pub mod request;
pub mod response;
pub mod route;

use futures;
use regex::Regex;
use request::Request;
use response::Response;
use route::{HttpMethod, Route};
use socket2::{Domain, Socket, Type};
use std::collections::BTreeMap;
use std::io;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use threadpool::ThreadPool;

pub type Handler = fn(request: &Request) -> response::Response;

pub struct HttpServer {
    port: i16,
    routes: BTreeMap<Route, Handler>,
    workers: usize,
}

impl HttpServer {
    pub fn bind(port: i16) -> HttpServer {
        HttpServer {
            port: port,
            routes: BTreeMap::new(),
            workers: 16,
        }
    }

    pub fn workers(mut self, workers: usize) -> HttpServer {
        self.workers = workers;
        self
    }

    pub fn get(self, endpoint: &str, handler: Handler) -> HttpServer {
        self.route(endpoint, HttpMethod::Get, handler)
    }

    pub fn post(self, endpoint: &str, handler: Handler) -> HttpServer {
        self.route(endpoint, HttpMethod::Post, handler)
    }

    fn route(mut self, endpoint: &str, method: HttpMethod, handler: Handler) -> HttpServer {
        let route = Route::new(&endpoint.to_string(), method);
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
        let runner = async {
            let (tx, rx): (Sender<TcpStream>, Receiver<TcpStream>) = mpsc::channel();
            let sender = listen_for_requests(self.port, tx);
            let receiver = respond_to_requests(self.routes, self.workers, rx);
            futures::join!(sender, receiver);
        };
        futures::executor::block_on(runner);
    }
}

async fn listen_for_requests(port: i16, tx: Sender<TcpStream>) {
    let s = Socket::new(Domain::ipv4(), Type::stream(), None).unwrap();
    s.bind(
        &format!("127.0.0.1:{}", port)
            .parse::<SocketAddr>()
            .unwrap()
            .into(),
    )
    .expect("Failed to bind to port.");
    s.set_nonblocking(true).expect("Non-blocking failed.");

    let listener = s.into_tcp_listener();
    let mut incoming = listener.incoming();
    while let Some(tcp_stream) = incoming.next() {
        match tcp_stream {
            Ok(s) => tx.send(s).expect("Failed to send TcpStream."),
            Err(e) => {
                if e.kind() != io::ErrorKind::WouldBlock {
                    panic!("Unexpected error");
                }
            }
        };
    }
}

async fn respond_to_requests(
    routes: BTreeMap<Route, Handler>,
    workers: usize,
    rx: Receiver<TcpStream>,
) {
    let pool = ThreadPool::new(workers);
    loop {
        let routes = routes.clone();
        let stream = rx.recv();
        pool.execute(move || {
            let request = Request::new("TODO GET /endpoint HTTP 1.1");
            let uri = &request.uri;
            for (route, handler) in routes {
                let regex = Regex::new(&route.regex).unwrap();
                if !regex.is_match(&uri) {
                    continue;
                }
                let response = handler(&request);
                // TODO somehow write to stream
            }
        });
    }
}
