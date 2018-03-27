extern crate hyper;

use hyper::{Method, StatusCode};
use hyper::server::{const_service, service_fn, Http, Request, Response};
use hyper::header::{ContentLength, ContentType};
use std::net::SocketAddr;

fn main() {
    let addr = "[::1]:3000".parse().expect("Failed to parse address");
    run_echo_server(&addr).expect("Failed to run web server");
}

fn run_echo_server(addr: &SocketAddr) -> Result<(), hyper::Error> {
    let echo_service = const_service(service_fn(|req: Request| {
        // An easy way to implement routing is
        // to simply match the request's path
        match (req.method(), req.path()) {
            (&Method::Get, "/") => handle_root(),
            (&Method::Post, "/echo") => handle_echo(req),
            _ => handle_not_found(),
        }
    }));

    let server = Http::new().bind(addr, echo_service)?;
    server.run()
}

type ResponseResult = Result<Response<hyper::Body>, hyper::Error>;
fn handle_root() -> ResponseResult {
    const MSG: &str = "Try doing a POST at /echo";
    Ok(Response::new()
        .with_header(ContentType::plaintext())
        .with_header(ContentLength(MSG.len() as u64))
        .with_body(MSG))
}

fn handle_echo(req: Request) -> ResponseResult {
    // The echoing is implemented by setting the response's
    // body to the request's body
    Ok(Response::new().with_body(req.body()))
}

fn handle_not_found() -> ResponseResult {
    // Return a 404 for every unsupported route
    Ok(Response::new().with_status(StatusCode::NotFound))
}
