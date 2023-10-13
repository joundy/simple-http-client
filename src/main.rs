use std::io::prelude::*;
use std::io::ErrorKind;
use std::net::TcpStream;
use std::time::Duration;
use std::{str, usize};

use rustls;
use webpki;
use webpki_roots;

const BUFF_SIZE: usize = 64;

// buffer version of "\r\n\r\n"
// used to split http response
const HTTP_SPLIT_BUFFER: [u8; 4] = [13, 10, 13, 10]; 

pub enum HttpVersion {
    HTTP10,
    HTTP11,
    HTTP20,
}

pub enum HTTPMethod {
    HEAD,
    GET,
}

struct RequestHeader {
    headers: Vec<String>,
}

impl RequestHeader {
    fn new(method: HTTPMethod, path: &str, http_version: HttpVersion) -> Self {
        let m = match method {
            HTTPMethod::GET => "GET",
            HTTPMethod::HEAD => "HEAD",
        };

        let v = match http_version {
            HttpVersion::HTTP10 => "HTTP/1.0",
            HttpVersion::HTTP11 => "HTTP/1.1",
            HttpVersion::HTTP20 => "HTTP/2.0",
        };

        let prefix = format!("{} {} {}", m, path, v);
        let mut headers = vec![];
        headers.push(prefix);

        RequestHeader { headers }
    }

    fn add(&mut self, key: &str, value: &str) {
        let header = format!("{}: {}", key, value);
        self.headers.push(header);
    }

    fn parse_as_str(&self) -> String {
        let mut headers_join = self.headers.join("\r\n");
        headers_join.push_str("\r\n\r\n");
        headers_join
    }
}

// find the last idx buffer the match other buffer
fn find_buffer_match_idx<T: PartialEq>(buffer_match: &[T], buffer: &[T]) -> Option<usize> {
    if buffer.len() < buffer_match.len(){
        return None;
    }

    for i in 0..=buffer.len() - buffer_match.len(){
        if buffer[i..(i + buffer_match.len())] == *buffer_match{
            return Some(i + buffer_match.len());
        }
    };

    return None
}

fn read_response<T>(mut reader: T) -> std::io::Result<(Vec<u8>, Vec<u8>)>
where
    T: Read,
{
    let mut response = vec![];

    // this could also be possible using stream.read_to_end, but i choose to construct the buffer by my
    // myself, learning the hard way sometimes gives you more knowledge.
    loop {
        let mut buffer = [0; BUFF_SIZE];
        let len = match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(len) => Some(len),
            Err(error) => match error.kind() {
                ErrorKind::ConnectionAborted => break,
                _ => panic!("errors.when reading buffer: {}", error),
            },
        };

        if len.unwrap() < BUFF_SIZE {
            response.write_all(&buffer[..len.unwrap()])?;
        } else {
            response.write_all(&buffer[..BUFF_SIZE])?;
        }
    }

    let match_idx = find_buffer_match_idx(&HTTP_SPLIT_BUFFER, &response);

    let mut header = vec![];
    let mut body = vec![];

    header.write_all(&response[0..match_idx.unwrap()])?;
    body.write_all(&response[match_idx.unwrap()..])?;

    Ok((header, body))
}

fn parse_response(response: Vec<u8>) {
    let response_str = String::from_utf8(response).unwrap();
    let parsed_response = response_str.split("\n").collect::<Vec<&str>>();

    dbg!(parsed_response);
}

fn main() -> std::io::Result<()> {
    let host = "example.com";
    // let port = 80; // http
    let port = 443; // https
    let mut stream = TcpStream::connect((host, port))?;
    stream.set_read_timeout(Some(Duration::from_secs(60)))?;
    stream.set_write_timeout(Some(Duration::from_secs(60)))?;

    let mut header = RequestHeader::new(HTTPMethod::GET, "/", HttpVersion::HTTP11);

    header.add("Host", host);
    header.add("Connection", "Close");

    if port == 443 {
        let mut config_tls = rustls::ClientConfig::new();
        config_tls
            .root_store
            .add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);
        let arc = std::sync::Arc::new(config_tls);

        let dns_name = webpki::DNSNameRef::try_from_ascii(host.as_bytes()).unwrap();
        let mut client = rustls::ClientSession::new(&arc, dns_name);

        let mut stream = rustls::Stream::new(&mut client, &mut stream);
        stream.write_all(header.parse_as_str().as_bytes())?;
        stream.flush()?;

        let response = read_response(stream)?;
        parse_response(response.0);
        parse_response(response.1);
    } else {
        stream.write_all(header.parse_as_str().as_bytes())?;
        stream.flush()?;

        let response = read_response(stream)?;
        parse_response(response.0);
        parse_response(response.1);
    }

    Ok(())
}
