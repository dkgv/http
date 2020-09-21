mod request;
mod response;
mod route;

use route::{HttpMethod, Route};
use std::collections::HashMap;

type Handler = fn(request: request::Request) -> response::Response;

struct HttpServer {
    port: i16,
    routes: HashMap<Route, Handler>
}

impl HttpServer {
    fn bind(port: i16) -> HttpServer {
        HttpServer {
            port: port,
            routes: HashMap::new()
        }
    }

    fn get(self, endpoint: &String, handler: Handler) -> HttpServer {
        self.route(endpoint, HttpMethod::Get, handler)
    }

    fn post(self, endpoint: &String, handler: Handler) -> HttpServer {
        self.route(endpoint, HttpMethod::Post, handler)
    }

    fn route(mut self, endpoint: &String, method: HttpMethod, handler: Handler) -> HttpServer {
        let route = Route::new(endpoint, method);
        if self.routes.contains_key(&route) {
            panic!("Unable to define duplicate route {} with specified HTTP method.", endpoint);
        }
        self.routes.insert(route, handler);
        self
    }

    fn listen() {
        // TODO socket
    }
}
