use crate::http::*;

pub enum HttpRequestMethod {
    Get,
}

impl FromStr for HttpRequestMethod {
    type Err = fmt::Error;

    // Required method
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(HttpRequestMethod::Get),
            _ => Err(fmt::Error),
        }
    }
}

impl fmt::Display for HttpRequestMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let version = match self {
            HttpRequestMethod::Get => "GET",
        };

        write!(f, "{}", version)
    }
}

pub struct HttpRequest {
    method: HttpRequestMethod,
    request_target: String,
    version: HttpVersion,
}

impl HttpRequest {
    pub fn new(method: HttpRequestMethod, request_target: String, version: HttpVersion) -> Self {
        Self {
            method,
            request_target,
            version,
        }
    }
}

impl fmt::Display for HttpRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {}\r\n\r\n",
            self.method, self.request_target, self.version
        )
    }
}
