extern crate futures;
extern crate hyper;

use hyper::{Method, StatusCode};
use hyper::server::{const_service, service_fn, Http, Request, Response};
use hyper::header::{ContentLength, ContentType};
use hyper::mime;
use futures::Future;
use futures::sync::oneshot;
use std::net::SocketAddr;
use std::thread;
use std::fs::File;
use std::io::{self, copy};

fn main() {
    let addr = "[::1]:3000".parse().expect("Failed to parse address");
    run_file_server(&addr).expect("Failed to run web server");
}

fn run_file_server(addr: &SocketAddr) -> Result<(), hyper::Error> {
    let file_service = const_service(service_fn(|req: Request| {
        // Setting up our routes
        match (req.method(), req.path()) {
            (&Method::Get, "/") => handle_root(),
            (&Method::Get, path) => handle_get_file(path),
            _ => handle_invalid_method(),
        }
    }));

    let server = Http::new().bind(addr, file_service)?;
    server.run()
}

// Because we don't want the entire server to block when serving a file,
// we are going to return a response wrapped in a future
type ResponseFuture = Box<Future<Item = Response, Error = hyper::Error>>;
fn handle_root() -> ResponseFuture {
    // Send the landing page
    send_file_or_404("index.html")
}

fn handle_get_file(file: &str) -> ResponseFuture {
    // Send whatever page was requested or fall back to a 404 page
    send_file_or_404(file)
}

fn handle_invalid_method() -> ResponseFuture {
    // Send a page telling the user that the method he used is not supported
    let response_future = send_file_or_404("invalid_method.html")
        // Set the correct status code
        .and_then(|response| Ok(response.with_status(StatusCode::MethodNotAllowed)));
    Box::new(response_future)
}

// Send a future containing a response with the requested file or a 404 page
fn send_file_or_404(path: &str) -> ResponseFuture {
    // Sanitize the input to prevent unwanted data access
    let path = sanitize_path(path);

    let response_future = try_to_send_file(&path)
        // try_to_send_file returns a future of Result<Response, io::Error>
        // turn it into a future of a future of Response with an error of hyper::Error
        .and_then(|response_result| response_result.map_err(|error| error.into()))
        // If something went wrong, send the 404 page instead
        .or_else(|_| send_404());
    Box::new(response_future)
}

// Return a requested file in a future of Result<Response, io::Error>
// to indicate whether it exists or not
type ResponseResultFuture = Box<Future<Item = Result<Response, io::Error>, Error = hyper::Error>>;
fn try_to_send_file(file: &str) -> ResponseResultFuture {
    // Prepend "files/" to the file
    let path = path_on_disk(file);
    // Load the file in a separate thread into memory.
    // As soon as it's done, send it back through a channel
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(err) => {
                println!("Failed to find file: {}", path);
                // Send error through channel
                tx.send(Err(err)).expect("Send error on file not found");
                return;
            }
        };

        // buf is our in-memory representation of the file
        let mut buf: Vec<u8> = Vec::new();
        match copy(&mut file, &mut buf) {
            Ok(_) => {
                println!("Sending file: {}", path);
                // Detect the content type by checking the file extension
                // or fall back to plaintext
                let content_type = get_content_type(&path).unwrap_or_else(ContentType::plaintext);
                let res = Response::new()
                    .with_header(ContentLength(buf.len() as u64))
                    .with_header(content_type)
                    .with_body(buf);
                // Send file through channel
                tx.send(Ok(res))
                    .expect("Send error on successful file read");
            }
            Err(err) => {
                // Send error through channel
                tx.send(Err(err)).expect("Send error on error reading file");
            }
        };
    });
    // Convert all encountered errors to hyper::Error
    Box::new(rx.map_err(|error| io::Error::new(io::ErrorKind::Other, error).into()))
}

fn send_404() -> ResponseFuture {
    // Try to send our 404 page
    let response_future = try_to_send_file("not_found.html").and_then(|response_result| {
        Ok(response_result.unwrap_or_else(|_| {
            // If the 404 page doesn't exist, sent fallback text instead
            const ERROR_MSG: &str = "Failed to find \"File not found\" page. How ironic\n";
            Response::new()
                .with_status(StatusCode::NotFound)
                .with_header(ContentLength(ERROR_MSG.len() as u64))
                .with_body(ERROR_MSG)
        }))
    });
    Box::new(response_future)
}

fn sanitize_path(path: &str) -> String {
    // Normalize the separators for the next steps
    path.replace("\\", "/")
        // Prevent the user from going up the filesystem
        .replace("../", "")
        // If the path comes straigh from the router, 
        // it will begin with a slash
        .trim_left_matches(|c| c == '/')
        // Remove slashes at the end as we only serve files
        .trim_right_matches(|c| c == '/')
        .to_string()
}

fn path_on_disk(path_to_file: &str) -> String {
    "files/".to_string() + path_to_file
}

fn get_content_type(file: &str) -> Option<ContentType> {
    // Check the file extension and return the respective MIME type
    let pos = file.rfind('.')? + 1;
    let mime_type = match &file[pos..] {
        "txt" => mime::TEXT_PLAIN_UTF_8,
        "html" => mime::TEXT_HTML_UTF_8,
        "css" => mime::TEXT_CSS,
        // This list can be extended for all types your server should support
        _ => return None,
    };
    Some(ContentType(mime_type))
}
