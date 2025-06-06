use std::fmt;

pub enum ResponseCode {
    OK,
}

impl fmt::Display for ResponseCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            ResponseCode::OK => "200 OK",
        };

        write!(f, "{}", message)
    }
}

pub enum Version {
    Http11,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let version = match self {
            Version::Http11 => "HTTP/1.1",
        };

        write!(f, "{}", version)
    }
}

pub struct ResponseMsg {
    version: Version,
    response_code: ResponseCode,
    _headers: (),
}

impl ResponseMsg {
    pub fn new(version: Version, response_code: ResponseCode) -> Self {
        Self {
            version,
            response_code,
            _headers: (),
        }
    }

    pub fn to_string(&self) -> String {
        let version = self.version.to_string();
        let response_code = self.response_code.to_string();

        format!("{version} {response_code}\r\n\r\n")
    }
}
