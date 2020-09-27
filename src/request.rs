use std::collections::HashMap;

pub trait UriParam {
    fn parse(data: &String) -> Self;
}

impl UriParam for String {
    fn parse(data: &String) -> String {
        String::from(data)
    }
}

impl UriParam for i32 {
    fn parse(data: &String) -> i32 {
        data.parse::<i32>().expect("Unable to parse i32.")
    }
}

impl UriParam for f32 {
    fn parse(data: &String) -> f32 {
        data.parse::<f32>().expect("Unable to parse f32.")
    }
}

#[derive(Clone)]
pub struct Request {
    pub uri: String,
    headers: HashMap<String, String>,
    params: HashMap<String, String>,
}

impl Request {
    pub fn new(request_line: &str) -> Request {
        let uri = String::from("");
        let headers = HashMap::new();
        let params = HashMap::new();

        Request {
            uri: uri,
            headers: headers,
            params: params,
        }
    }

    pub fn get_header(&self, header: &str) -> String {
        match self.headers.get(header) {
            Some(val) => val.to_string(),
            None => String::default(),
        }
    }

    pub fn get_param<T: UriParam>(&self, name: &str) -> T {
        if !self.params.contains_key(name) {
            panic!("Invalid parameter {}", name);
        }
        UriParam::parse(self.params.get(name).unwrap())
    }
}
