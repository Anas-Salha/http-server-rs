use crate::http::{
    response::{HttpResponse, HttpResponseCode},
    *,
};

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
    target: String,
    version: HttpVersion,
}

impl HttpRequest {
    pub fn new(method: HttpRequestMethod, target: String, version: HttpVersion) -> Self {
        Self {
            method,
            target,
            version,
        }
    }

    pub fn execute_method(&self) -> HttpResponse {
        match self.method {
            HttpRequestMethod::Get => return self.get(),
        }
    }

    fn get(&self) -> HttpResponse {
        let root = std::fs::canonicalize(".").unwrap(); // Set project root as the root directory to search within
        let candidate = self.target.trim_start_matches('/');
        let candidate = root.join(candidate);
        let real = match std::fs::canonicalize(candidate) {
            Ok(n) => n,
            Err(e) => {
                eprintln!("{}", e);
                return HttpResponse::new(HttpVersion::Http11, HttpResponseCode::NotFound);
            }
        };

        // Any path outside the specified root will be treated as non-existent to avoid path traversal attacks
        // see: https://owasp.org/www-community/attacks/Path_Traversal
        if !real.starts_with(root) {
            return HttpResponse::new(HttpVersion::Http11, HttpResponseCode::NotFound);
        }

        return HttpResponse::new(HttpVersion::Http11, HttpResponseCode::Ok);
    }
}

impl fmt::Display for HttpRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {}\r\n\r\n",
            self.method, self.target, self.version
        )
    }
}
