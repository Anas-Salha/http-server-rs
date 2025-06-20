use std::{fmt, str::FromStr};
pub mod request;
pub mod response;

#[derive(Clone)]
pub enum HttpVersion {
    Http11,
}

impl fmt::Display for HttpVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let version = match self {
            HttpVersion::Http11 => "HTTP/1.1",
        };

        write!(f, "{}", version)
    }
}

impl FromStr for HttpVersion {
    type Err = fmt::Error;

    // Required method
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HTTP/1.1" => Ok(HttpVersion::Http11),
            _ => Err(fmt::Error),
        }
    }
}

#[derive(Clone)]
pub enum HttpHeader {
    ContentType(mime::Mime),
    ContentLength(u64),
    Host(String),
    UserAgent(String),
    Accept(String),
}

impl fmt::Display for HttpHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpHeader::ContentType(mime) => write!(f, "Content-Type: {}\r\n", mime),
            HttpHeader::ContentLength(len) => write!(f, "Content-Length: {}\r\n", len),
            HttpHeader::Host(s) => write!(f, "Host: {}\r\n", s),
            HttpHeader::UserAgent(s) => write!(f, "User-Agent: {}\r\n", s),
            HttpHeader::Accept(s) => write!(f, "Accept: {}\r\n", s),
        }
    }
}
