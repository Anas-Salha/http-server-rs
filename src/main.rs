use http::request::*;
use mime::Mime;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use crate::http::HttpHeader;

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
    let req_line_end = buf
        .windows(2)
        .position(|w| w == b"\r\n")
        .ok_or("Failed to find request line end")?;
    let req_line = std::str::from_utf8(&buf[..req_line_end])?;

    let mut fields = req_line.split_whitespace();
    let method = fields.next().ok_or("Failed to parse method")?.parse()?;
    let target = fields.next().ok_or("Failed to parse target")?.to_owned();
    let version = fields.next().ok_or("Failed to parse version")?.parse()?;

    // 2. Read headers
    let headers_end = buf
        .windows(4)
        .position(|w| w == b"\r\n\r\n")
        .ok_or("Failed to find headers end")?;

    let headers_start = req_line_end + 2; //skip CRLF after req_line
    let headers: Vec<&str> = std::str::from_utf8(&buf[headers_start..headers_end])?
        .split("\r\n")
        .collect();

    let headers: Vec<HttpHeader> = headers
        .iter()
        .filter_map(|header| parse_header(header))
        .collect();

    Ok(HttpRequest::new(method, target, version, headers))

    // 3. Read (optional) body
}

fn parse_header(header: &str) -> Option<HttpHeader> {
    let parts: Vec<&str> = header.splitn(2, ":").collect();
    assert_eq!(parts.len(), 2);

    let key = parts[0].to_lowercase();
    let value = parts[1].trim();

    match key.as_str() {
        "content-type" => value
            .parse::<mime::Mime>()
            .ok()
            .map(HttpHeader::ContentType),
        "content-length" => value.parse::<u64>().ok().map(HttpHeader::ContentLength),
        "host" => Some(HttpHeader::Host(value.to_string())),
        "user-agent" => Some(HttpHeader::UserAgent(value.to_string())),
        "accept" => Some(HttpHeader::Accept(value.to_string())),
        _ => None,
    }
}
