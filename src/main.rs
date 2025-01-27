extern crate http_from_scratch;

use http_from_scratch::{
    request::Request,
    response::{Response, Status},
};

use std::io::Write;
use std::net::{TcpListener, TcpStream};

fn handle_connection(mut stream: TcpStream) {
    let req = Request::from_reader(&mut stream);

    let mut resp = Response::new(Status::Ok);
    if let Some(body) = req.body {
        resp = resp.with_body(&body);
    }

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
