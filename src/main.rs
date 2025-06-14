use http::request::*;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

mod http;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                match read_request(&mut stream) {
                    Ok(req) => {
                        let resp = req.respond();

                        // Respond
                        let msg = resp.to_string();
                        if let Err(e) = stream.write_all(msg.as_bytes()) {
                            eprintln!("Failed to write to stream: {}", e);
                        };
                        if let Err(e) = stream.flush() {
                            eprintln!("Failed to flush stream: {}", e);
                        };
                    }
                    Err(e) => {
                        eprintln!("Failed to read request: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

// ref: https://datatracker.ietf.org/doc/html/rfc9112#name-message-parsing
// The normal procedure for parsing an HTTP message is to read the start-line into a structure,
// read each header field line into a hash table by field name until the empty line,
// and then use the parsed data to determine if a message body is expected.
//
// If a message body has been indicated,
// then it is read as a stream until an amount of octets equal to the message body length is read or the connection is closed.
fn read_request(stream: &mut TcpStream) -> Result<HttpRequest, Box<dyn std::error::Error>> {
    // Setup buffer to read stream
    let mut buf = [0u8; 4096];
    stream.read(&mut buf)?;

    // 1. Read request line
    let line_end = buf
        .windows(2)
        .position(|w| w == b"\r\n")
        .ok_or("Failed to find request line end")?;
    let line = std::str::from_utf8(&buf[..line_end])?;

    let mut fields = line.split_whitespace();
    let method = fields.next().ok_or("Failed to parse method")?.parse()?;
    let target = fields.next().ok_or("Failed to parse target")?.to_owned();
    let version = fields.next().ok_or("Failed to parse version")?.parse()?;

    Ok(HttpRequest::new(method, target, version))

    // 2. Read headers
    // 3. Read (optional) body
}
