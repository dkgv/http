use std::collections::HashMap;

trait UriParam {
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

pub struct Request {
    uri: String,
    headers: HashMap<String, String>,
    params: HashMap<String, String>
}

impl Request {
    fn get_header(self, header: String) -> String {
        match self.headers.get(&header) {
            Some(val) => val.to_string(),
            None => String::default()
        }
    }

    fn get<T: UriParam>(self, name: String) -> T {
        if !self.params.contains_key(&name) {
            panic!("Invalid parameter {}", name);
        }
        UriParam::parse(self.params.get(&name).unwrap())
    }
}
