extern crate http_from_scratch;

use http_from_scratch::{
    common::Method,
    request::Request,
    response::{Response, Status},
    router::{Params, Router},
};

use std::net::TcpListener;
use std::{io::Write, sync::RwLock};

fn handle_echo(req: Request, _: &Params, state: &RwLock<usize>) -> Response {
    {
        let mut count = state.write().unwrap();
        *count += 1;

        println!("Echo endpoint has been hit {count} times.");
    }

    let mut resp = Response::new(Status::Ok);
    if let Some(body) = req.body {
        resp = resp.with_body(&body);
    }

    return resp;
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    // Set up state and router
    let request_counter: RwLock<usize> = RwLock::new(0);
    let mut router = Router::<RwLock<usize>>::new(request_counter);
    router.add(Method::Post, "/echo", handle_echo);

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let req = Request::from_reader(&mut stream);
        let resp = router
            .handle(req)
            .unwrap_or(Response::new(Status::NotFound));

        stream.write_all(resp.to_string().as_bytes()).unwrap();
    }
}
