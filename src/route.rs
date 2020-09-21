use regex::Regex;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash)]
pub enum HttpMethod {
    Get, Post
}

#[derive(Eq, PartialEq, Hash)]
pub enum ParamType {
    Str, Int, Float
}

impl ParamType {
    fn regex(self) -> Regex {
        let regex = match self {
            ParamType::Str => r".+",
            ParamType::Int => r"[0-9]+",
            ParamType::Float => r"[-+]?[0-9]*\.?[0-9]+"
        };
        Regex::new(regex).unwrap()
    }
}

#[derive(Eq, PartialEq, Hash)]
pub struct Route {
    endpoint: String,
    method: HttpMethod,
    params: HashMap<String, ParamType>
}

impl Route {
    pub fn new(endpoint: &String, method: HttpMethod) -> Route {
        let regex = Regex::new(r"<([A-Za-z]+):([A-Za-z]+)>").unwrap();
        let mut params: HashMap<String, ParamType> = HashMap::new();

        for group in regex.captures_iter(endpoint) {
            let param_name = &group[0];
            let param_type = match &group[1] {
                "int" => ParamType::Int,
                "str" => ParamType::Str,
                "float" => ParamType::Float,
                _ => ParamType::Str
            };
            params.insert(param_name.to_string(), param_type);
        }

        Route {
            endpoint: endpoint.to_owned(),
            method: method,
            params: params
        }
    }
}
