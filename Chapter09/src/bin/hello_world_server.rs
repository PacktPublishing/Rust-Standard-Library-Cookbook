extern crate futures;
extern crate hyper;

use futures::future::Future;
use hyper::header::{ContentLength, ContentType};
use hyper::server::{const_service, service_fn, Http, Request, Response, Service};
use std::net::SocketAddr;

const MESSAGE: &str = "Hello World!";

fn main() {
    // [::1] is the loopback address for IPv6, 3000 is a port
    let addr = "[::1]:3000".parse().expect("Failed to parse address");
    run_with_service_function(&addr).expect("Failed to run web server");
}

fn run_with_service_function(addr: &SocketAddr) -> Result<(), hyper::Error> {
    // Hyper is based on Services, which are construct that
    // handle how to respond to requests.
    // const_service and service_fn are convenience functions
    // that build a service out of a closure
    let hello_world = const_service(service_fn(|_| {
        println!("Got a connection!");
        // Return a Response with a body of type hyper::Body
        Ok(Response::<hyper::Body>::new()
            // Add header specifying content type as plain text
            .with_header(ContentType::plaintext())
            // Add header specifying the length of the message in bytes
            .with_header(ContentLength(MESSAGE.len() as u64))
            // Add body with our message
            .with_body(MESSAGE))
    }));

    let server = Http::new().bind(addr, hello_world)?;
    server.run()
}

// The following function does the same, but uses an explicitely created
// struct HelloWorld that implements the Service trait
fn run_with_service_struct(addr: &SocketAddr) -> Result<(), hyper::Error> {
    let server = Http::new().bind(addr, || Ok(HelloWorld))?;
    server.run()
}

struct HelloWorld;
impl Service for HelloWorld {
    // Implementing a server requires specifying all involved types
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    // The future that wraps your eventual Response
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, _: Request) -> Self::Future {
        // In contrast to service_fn, we need to explicitely return a future
        Box::new(futures::future::ok(
            Response::new()
                .with_header(ContentType::plaintext())
                .with_header(ContentLength(MESSAGE.len() as u64))
                .with_body(MESSAGE),
        ))
    }
}
