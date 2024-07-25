use std::collections::HashMap;

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