use std::{collections::HashMap, os::windows::process};

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized
}

impl From<&str> for Method {
    fn from(s: &str) -> Method {
        match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized
}

impl From<&str> for Version {
    fn from(s: &str) -> Version {
        match s {
            "HTTP/1.1" => Version::V1_1,
            "HTTP/2" => Version::V2_0, 
            _ => Version::Uninitialized
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = "".to_string();
    }

    for line in req.lines() {
        if line.contains("HTTP") {
            let (method, resource, version) = process_req_line(line);
            parsed_method = method;
            parsed_version = version;
            parsed_resouce = resouce;
            process_header_line();
        } else if line.contains(":") {
            let (key, value) = process_header_lineeeeeeeeeeeeeeeeeeeeeeeeeeeeeel 
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_method_into() {
        let m: Method = "GET".into();
        let n: Method = "POST".into();
        let o: Method = "".into();
        assert_eq!(m, Method::Get);
        assert_eq!(n, Method::Post);
        assert_eq!(o, Method::Uninitialized);
    }

    #[test]
    fn test_version_into() {
        let m:Version = "HTTP/1.1".into();
        assert_eq!(m, Version::V1_1);
        let n:Version = "HTTP/2".into();
        assert_eq!(n, Version::V2_0);
        let o:Version = "".into();
        assert_eq!(o, Version::Uninitialized);
    }
}