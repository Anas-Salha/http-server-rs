use std::{fmt, str::FromStr};
pub mod request;
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
