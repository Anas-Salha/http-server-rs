use http::response::*;
use http::*;
use std::{io::Write, net::TcpListener};

mod http;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                let msg = HttpResponse::new(HttpVersion::Http11, HttpResponseCode::OK).to_string();

                if let Err(e) = stream.write_all(msg.as_bytes()) {
                    eprintln!("Failed to write to stream: {}", e);
                };

                if let Err(e) = stream.flush() {
                    eprintln!("Failed to flush stream: {}", e);
                };
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
