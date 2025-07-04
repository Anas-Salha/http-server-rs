use http::request::*;
use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use crate::http::HttpHeader;

mod http;
use clap::Parser;
use std::sync::OnceLock;

pub static DIR: OnceLock<String> = OnceLock::new();

#[derive(Parser, Debug)]
struct Args {
    /// Path of the directory to get files from
    #[arg(long, value_name = "PATH", default_value_t = String::from("/test"))]
    directory: String,
}

fn main() {
    let args = Args::parse();
    DIR.set(args.directory).unwrap();
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                std::thread::spawn(move || {
                    handle_connection(&mut stream);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(stream: &mut TcpStream) {
    println!("accepted new connection");
    match read_request(stream) {
        Ok(req) => {
            let resp = req.respond();

            // Respond
            let msg = resp.to_string();
            let mut bytes = msg.into_bytes();
            bytes.extend_from_slice(&resp.get_body());
            if let Err(e) = stream.write_all(&bytes) {
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

// ref: https://datatracker.ietf.org/doc/html/rfc9112#name-message-parsing
// The normal procedure for parsing an HTTP message is to read the start-line into a structure,
// read each header field line into a hash table by field name until the empty line,
// and then use the parsed data to determine if a message body is expected.
//
// If a message body has been indicated,
// then it is read as a stream until an amount of octets equal to the message body length is read or the connection is closed.
fn read_request(stream: &mut TcpStream) -> Result<HttpRequest, Box<dyn std::error::Error>> {
    // Setup buffer to read stream
    let mut reader = BufReader::new(stream);

    // 1. Read request line
    let mut req_line = String::new();
    reader.read_line(&mut req_line)?;
    let req_line = req_line.trim_end_matches("\r\n");
    let mut fields = req_line.split_whitespace();
    let method = fields.next().ok_or("Failed to parse method")?.parse()?;
    let target = fields.next().ok_or("Failed to parse target")?.to_owned();
    let version = fields.next().ok_or("Failed to parse version")?.parse()?;

    // 2. Read headers
    let mut headers = vec![];
    loop {
        let mut header_line = String::new();
        reader.read_line(&mut header_line)?;
        let header_line = header_line.trim_end_matches("\r\n");
        if header_line.is_empty() {
            break;
        }

        if let Some(header) = parse_header(header_line) {
            headers.push(header);
        }
    }

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
