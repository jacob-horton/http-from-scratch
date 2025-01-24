use std::collections::HashMap;
use std::io::{BufRead, Read, Write};
use std::str::FromStr;
use std::{
    io::BufReader,
    net::{TcpListener, TcpStream},
};

#[derive(Debug, Clone, PartialEq)]
enum Method {
    Get,
    Post,
}

impl FromStr for Method {
    type Err = ();

    fn from_str(input: &str) -> Result<Method, Self::Err> {
        match input {
            "GET" => Ok(Method::Get),
            "POST" => Ok(Method::Post),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
struct Request {
    method: Method,
    path: String,
    version: String,
    headers: HashMap<String, String>,
    body: Option<String>,
}

impl Request {
    fn from_reader(stream: &mut TcpStream) -> Self {
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

#[derive(Debug, Clone)]
enum Status {
    Ok,
    BadRequest,
    NotFound,
}

impl ToString for Status {
    fn to_string(&self) -> String {
        match (self) {
            Self::Ok => "200 OK",
            Self::BadRequest => "400 Bad Request",
            Self::NotFound => "404 Not Found",
        }
        .to_string()
    }
}

#[derive(Debug, Clone)]
struct Response {
    version: String,
    status_code: Status,
    headers: HashMap<String, String>,
    body: Option<String>,
}

impl ToString for Response {
    fn to_string(&self) -> String {
        let mut result = String::new();
        result.push_str(&self.version);
        result.push_str(" ");
        result.push_str(&self.status_code.to_string());
        result.push_str("\r\n");

        for (k, v) in &self.headers {
            // This is calculatd and added later
            if k.to_lowercase() == "content-length" {
                continue;
            }

            result.push_str(&k.to_string());
            result.push_str(": ");
            result.push_str(&v.to_string());
            result.push_str("\r\n");
        }

        if let Some(body) = &self.body {
            result.push_str("Content-Lenth: ");
            result.push_str(&body.len().to_string());

            result.push_str("\r\n\r\n");
            result.push_str(body);
        } else {
            result.push_str("Content-Lenth: 0");
        }

        return result;
    }
}

fn handle_connection(mut stream: TcpStream) {
    let req = Request::from_reader(&mut stream);

    println!("{req:#?}");

    let resp = Response {
        version: "HTTP/1.1".to_string(),
        status_code: Status::Ok,
        headers: HashMap::new(),
        body: req.body,
    };

    println!("{}", resp.to_string());

    stream.write_all(resp.to_string().as_bytes()).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}
