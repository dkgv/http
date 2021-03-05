use std::collections::BTreeMap;

pub struct Response {
    pub headers: BTreeMap<String, String>,
    pub body: String,
}

impl Response {
    pub fn new(body: &str) -> Response {
        let resp = Response {
            headers: BTreeMap::new(),
            body: body.to_string()
        };
        resp.add_header("Content-Type", "text/html; charset=UTF-8").status(200)
    }

    pub fn status(self, status: i16) -> Response {
        self.add_header("Status", &[status.to_string(), http_message(status)].join(" "))
    }

    pub fn content_type(self, content_type: &str) -> Response {
        self.add_header("Content-Type", content_type)
    }

    pub fn add_header(mut self, key: &str, value: &str) -> Response {
        self.headers.insert(String::from(key), String::from(value));
        self
    }

    pub fn dump(self) -> String {
        let mut sb:Vec<String> = Vec::new();
        sb.push(["HTTP/1.1", self.headers.get("Status").unwrap()].join(" "));

        for header in self.headers {
            sb.push([&header.0, ":", &header.1].join(" "));
        }

        sb.push(self.body);
        sb.join("\n")
    }
}

fn http_message(code: i16) -> String {
    let message = match code {
        100 => "Continue",
        101 => "Switching Protocols",
        102 => "Processing",
        200 => "OK",
        201 => "Created",
        202 => "Accepted",
        203 => "Non-Authoritative Information",
        204 => "No Content",
        205 => "Reset Content",
        206 => "Partial Content",
        207 => "Multi-Status",
        208 => "Already Reported",
        226 => "IM Used",
        300 => "Multiple Choices",
        301 => "Moved Permanently",
        302 => "Found",
        303 => "See Other",
        304 => "Not Modified",
        305 => "Use Proxy",
        306 => "Switch Proxy",
        307 => "Temporary Redirect",
        308 => "Permanent Redirect",
        400 => "Bad Request",
        401 => "Unauthorized",
        402 => "Payment Required",
        403 => "Forbidden",
        404 => "Not Found",
        405 => "Method Not Allowed",
        406 => "Not Acceptable",
        407 => "Proxy Authentication Required",
        408 => "Request Timeout",
        409 => "Conflict",
        410 => "Gone",
        411 => "Length Required",
        412 => "Precondition Failed",
        413 => "Payload Too Large",
        414 => "URI Too Long",
        415 => "Unsupported Media Type",
        416 => "Range Not Satisfiable",
        417 => "Expectation Failed",
        421 => "Misdirected Request",
        422 => "Unprocessable Entity",
        423 => "Locked",
        424 => "Failed Dependency",
        426 => "Upgrade Required",
        428 => "Precondition Required",
        429 => "Too Many Requests",
        431 => "Request Header Fields Too Large",
        451 => "Unavailable For Legal Reasons",
        500 => "Internal Server Error",
        501 => "Not Implemented",
        502 => "Bad Gateway",
        503 => "Service Unavailable",
        504 => "Gateway Timeout",
        505 => "HTTP Version Not Supported",
        _ => "OK",
    };
    String::from(message)
}
