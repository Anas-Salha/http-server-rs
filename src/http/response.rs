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
    headers: Vec<HttpHeader>,
    body: String,
}

impl HttpResponse {
    pub fn new(
        version: HttpVersion,
        response_code: HttpResponseCode,
        headers: Vec<HttpHeader>,
        body: String,
    ) -> Self {
        Self {
            version,
            response_code,
            headers,
            body,
        }
    }
}

impl fmt::Display for HttpResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}\r\n", self.version, self.response_code)?;

        self.headers
            .iter()
            .try_for_each(|header| write!(f, "{}", header))?;
        write!(f, "\r\n")?;

        if !self.body.is_empty() {
            write!(f, "{}", self.body)
        } else {
            Ok(())
        }
    }
}
