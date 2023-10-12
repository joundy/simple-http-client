use std::io::prelude::*;
use std::net::TcpStream;
use std::time::Duration;
use std::{str, usize};

const BUFF_SIZE: usize = 8 * 8;

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

fn read_response(mut stream: &TcpStream) -> std::io::Result<Vec<u8>> {
    let mut response = vec![];

    // this could also be possible using stream.read_to_end, but i choose to construct the buffer by my
    // myself, learning the hard way sometimes gives you more knowledge.
    loop {
        let mut buffer = [0; BUFF_SIZE];
        let len = match stream.read(&mut buffer) {
            Ok(0) => None,
            Ok(len) => Some(len),
            Err(_) => panic!("errors.when reading buffer response"),
        };

        if len.is_none() {
            break;
        }

        if len.unwrap() < BUFF_SIZE {
            response.write_all(&buffer[..len.unwrap()])?;
        } else {
            response.write_all(&buffer[..BUFF_SIZE])?;
        }
    }

    Ok(response)
}

fn main() -> std::io::Result<()> {
    let host = "example.com";
    let port = 80;
    let mut stream = TcpStream::connect((host, port))?;

    let mut header = RequestHeader::new(HTTPMethod::GET, "/", HttpVersion::HTTP11);

    header.add("Host", host);
    header.add("Connection", "Close");

    stream.set_read_timeout(Some(Duration::from_secs(60)))?;
    stream.set_write_timeout(Some(Duration::from_secs(60)))?;

    stream.write_all(header.parse_as_str().as_bytes())?;
    stream.flush()?;

    let response = read_response(&stream)?;
    let response_str = String::from_utf8(response).unwrap();

    let parsed_response = response_str.split("\n").collect::<Vec<&str>>();

    dbg!(parsed_response);

    Ok(())
}
