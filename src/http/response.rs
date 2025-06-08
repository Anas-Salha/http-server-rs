use crate::http::*;
use std::fmt;

pub enum HttpResponseCode {
    Ok,
    NotFound,
}

impl fmt::Display for HttpResponseCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            HttpResponseCode::Ok => "200 OK",
            HttpResponseCode::NotFound => "404 Not Found",
        };

        write!(f, "{}", message)
    }
}

pub struct HttpResponse {
    version: HttpVersion,
    response_code: HttpResponseCode,
}

impl HttpResponse {
    pub fn new(version: HttpVersion, response_code: HttpResponseCode) -> Self {
        Self {
            version,
            response_code,
        }
    }
}

impl fmt::Display for HttpResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}\r\n\r\n", self.version, self.response_code)
    }
}
