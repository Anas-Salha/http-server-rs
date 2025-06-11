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
                let req = read_request(&mut stream);
                let resp = req.execute_method();

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
fn read_request(stream: &mut TcpStream) -> HttpRequest {
    // Setup buffer to read stream
    let mut buf = [0u8; 4096];
    let _ = stream.read(&mut buf).unwrap(); // expect the line in a single read

    // 1. Read request line
    let line_end = buf.windows(2).position(|w| w == b"\r\n").unwrap();
    let line = std::str::from_utf8(&buf[..line_end]).unwrap();

    let mut fields = line.split_whitespace();
    let method = fields.next().unwrap().parse().unwrap();
    let target = fields.next().unwrap().to_owned();
    let version = fields.next().unwrap().parse().unwrap();

    HttpRequest::new(method, target, version)

    // 2. Read headers
    // 3. Read (optional) body
}
