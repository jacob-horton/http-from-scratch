use std::collections::HashMap;
use std::io::{BufRead, Read};
use std::str::FromStr;
use std::{io::BufReader, net::TcpStream};

use crate::common::Method;

#[derive(Debug, Clone)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl Request {
    pub fn from_reader(stream: &mut TcpStream) -> Self {
        let mut buf_read = BufReader::new(stream);

        let mut buf = String::new();

        // Request line
        buf_read.read_line(&mut buf).unwrap();
        let mut parts = buf.split(" ");
        let method = Method::from_str(parts.next().unwrap().trim()).unwrap();
        let path = parts.next().unwrap().trim().to_string();
        let version = parts.next().unwrap().trim().to_string();

        // Start of headers
        buf.clear();
        buf_read.read_line(&mut buf).unwrap();

        let mut headers = HashMap::new();
        while !buf.trim().is_empty() {
            let (key, value) = buf.split_once(":").unwrap();
            headers.insert(
                key.trim().to_owned().to_lowercase(),
                value.trim().to_owned(),
            );

            buf.clear();
            buf_read.read_line(&mut buf).unwrap();
        }

        let mut body = None;
        if let Some(length) = headers.get("content-length") {
            let length: usize = length.parse().unwrap();

            if length > 0 {
                let mut buf = vec![0; length];
                buf_read.read_exact(&mut buf).unwrap();

                let body_str = String::from_utf8(buf).unwrap();
                body = Some(body_str.trim().to_owned());
            }
        }

        return Request {
            body,
            headers,
            path,
            method,
            version,
        };
    }
}
