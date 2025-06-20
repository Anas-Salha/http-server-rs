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
    headers: Vec<HttpHeader>,
}

impl HttpRequest {
    pub fn new(
        method: HttpRequestMethod,
        target: String,
        version: HttpVersion,
        headers: Vec<HttpHeader>,
    ) -> Self {
        Self {
            method,
            target,
            version,
            headers,
        }
    }

    pub fn respond(&self) -> HttpResponse {
        match self.method {
            HttpRequestMethod::Get => return self.get(),
        }
    }

    fn get(&self) -> HttpResponse {
        let endpoint = self
            .target
            .strip_prefix('/')
            .unwrap()
            .split('/')
            .next()
            .unwrap();

        match endpoint {
            "echo" => self.echo(),
            "user-agent" => self.user_agent(),
            _ => self.get_static(),
        }
    }

    fn echo(&self) -> HttpResponse {
        let body = self.target.trim_start_matches("/echo/");
        let headers = vec![
            HttpHeader::ContentType(mime::TEXT_PLAIN),
            HttpHeader::ContentLength(body.len() as u64),
        ];
        HttpResponse::new(
            self.version.clone(),
            HttpResponseCode::Ok,
            headers,
            body.to_string(),
        )
    }

    fn user_agent(&self) -> HttpResponse {
        let body: String = self
            .headers
            .clone()
            .into_iter()
            .filter_map(|h| match h {
                HttpHeader::UserAgent(s) => Some(s),
                _ => None,
            })
            .collect();

        let headers = vec![
            HttpHeader::ContentType(mime::TEXT_PLAIN),
            HttpHeader::ContentLength(body.len() as u64),
        ];
        HttpResponse::new(
            self.version.clone(),
            HttpResponseCode::Ok,
            headers,
            body.to_string(),
        )
    }

    fn get_static(&self) -> HttpResponse {
        let root = std::fs::canonicalize(".").unwrap(); // Set project root as the root directory to search within
        let candidate = self.target.trim_start_matches('/');
        let candidate = root.join(candidate);

        // If the candidate path cannot be resolved, we treat it as non-existent and return a 404 Not Found response
        let real = match std::fs::canonicalize(candidate) {
            Ok(n) => n,
            Err(e) => {
                eprintln!("{}", e);
                return HttpResponse::new(
                    self.version.clone(),
                    HttpResponseCode::NotFound,
                    vec![],
                    String::new(),
                );
            }
        };

        // Any path outside the specified root will be treated as non-existent to avoid path traversal attacks
        // see: https://owasp.org/www-community/attacks/Path_Traversal
        if !real.starts_with(root) {
            return HttpResponse::new(
                self.version.clone(),
                HttpResponseCode::NotFound,
                vec![],
                String::new(),
            );
        }

        return HttpResponse::new(
            self.version.clone(),
            HttpResponseCode::Ok,
            vec![],
            String::new(),
        );
    }
}

impl fmt::Display for HttpRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}\r\n", self.method, self.target, self.version).and_then(|_| {
            for header in &self.headers {
                write!(f, "{}\r\n", header)?;
            }
            write!(f, "\r\n")
        })
    }
}
