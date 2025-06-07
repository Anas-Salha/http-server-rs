use crate::http::*;
use std::fmt;

pub enum HttpResponseCode {
    OK,
}

impl fmt::Display for HttpResponseCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            HttpResponseCode::OK => "200 OK",
        };

        write!(f, "{}", message)
    }
}

pub struct HttpResponseMsg {
    version: HttpVersion,
    response_code: HttpResponseCode,
    _headers: (),
}

impl HttpResponseMsg {
    pub fn new(version: HttpVersion, response_code: HttpResponseCode) -> Self {
        Self {
            version,
            response_code,
            _headers: (),
        }
    }
}

impl fmt::Display for HttpResponseMsg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}\r\n\r\n", self.version, self.response_code)
    }
}
