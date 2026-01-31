# 🌐 HTTP From Scratch

A bare-bones HTTP/1.1 server implementation written in Rust.

[Read the full write-up on my blog!](https://jacobhorton.dev/posts/2025/http-from-scratch/)

This project is an exploration into the HTTP protocol. It was built to understand how libraries like axum or actix-web work under the hood by implementing the parsing logic, TCP handling, and routing from the ground up.

> [!WARNING]
> This is an educational project. It does not cover the full HTTP spec, is not optimized for security or performance, and should not be used in production.


## ✨ Features

- TCP listener - manually handles incoming TCP streams
- Request parsing - reads directly from the stream to parse methods, paths, versions, and headers
- Body handling - supports `Content-Length` based body reading
- Cookie parsing - automatically parses `Cookie` headers into a usable format
- Router - a custom routing system to map HTTP methods and paths to specific handler functions


## 🚀 Usage

- `lib` is where the library code is - this is the actual implementation of the HTTP server
- `src` includes an example echo server


### Running the Example

To see the server in action, run the included echo server:
```bash
cargo run
```


## Library Example

Here is a snippet showing how to use the library:

```rust
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
```


## 📝 Limitations

As this was a learning exercise, several features are intentionally missing or simplified:
- Only supports HTTP/1.1
- No support for keep-alive (connections are closed after response)
- No support for chunked transfer encoding
- Single-threaded request handling (no thread pool)
