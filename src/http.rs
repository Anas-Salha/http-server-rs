use std::fmt;
pub mod response;

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
