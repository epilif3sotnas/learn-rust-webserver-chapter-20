// rust
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};

// internal
mod methods;

use crate::methods::default_path::{delete_method, get_method, post_method, put_method};

// external
use threadpool::ThreadPool;

const HTTP_VERSIONS: [&'static str; 3] = ["1", "1.1", "2"];
const HTTP_PROTOCOL: &'static str = "HTTP";

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("localhost:8080")?;
    println!("Listening: {:?}", listener.local_addr()?);

    let pool = ThreadPool::new(3);

    for stream in listener.incoming() {
        println!("Stream: {:?}", stream);
        println!("Something connected");

        let tcp_stream = stream?;
        pool.execute(|| {
            handle_connection(tcp_stream);
        });
    }
    Ok(())
}

// ### TODO - Think about send back errors ###
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    // ### TODO - robust line unwrap ###
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Got HTTP request: {:?}", http_request);

    let response = check_request(http_request);

    stream.write_all(response.as_bytes()).unwrap_or_else(|err| {
        eprintln!("Error: {:?}", err);
    });

    stream
        .flush()
        .unwrap_or_else(|err| eprintln!("Error: {:?}", err));
}

fn check_request(request: Vec<String>) -> String {
    if request.len() <= 0 {
        return String::from("HTTP/1.1 400 BAD REQUEST\r\n\r\n");
    }

    let req = &request[0];

    let req_divided: Vec<&str> = req.split(" ").collect();

    if req_divided.len() != 3 || !req_divided[2].contains(HTTP_PROTOCOL) {
        return String::from("HTTP/1.1 400 BAD REQUEST\r\n\r\n");
    }

    let mut contain_version = false;
    for version in HTTP_VERSIONS {
        if req_divided[2].contains(&version) {
            contain_version = true;
            break;
        }
    }

    if !contain_version {
        return String::from("HTTP/1.1 505 HTTP VERSION NOT SUPPORTED\r\n\r\n");
    }

    return distribute_with_verb_path(req_divided[0], req_divided[1]);
}

fn distribute_with_verb_path(verb: &str, path: &str) -> String {
    if verb.is_empty() || path.is_empty() {
        return String::from("HTTP/1.1 400 BAD REQUEST\r\n\r\n");
    }

    if path != "/" {
        return String::from("HTTP/1.1 404 NOT FOUND\r\n\r\n");
    }

    match verb {
        "GET" => return get_method(),
        "POST" => return post_method(),
        "PUT" => return put_method(),
        "DELETE" => return delete_method(),
        _ => return String::from("HTTP/1.1 404 NOT FOUND\r\n\r\n"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit__check_request__empty_vector() {
        let request: Vec<String> = Vec::new();

        let result = check_request(request);

        let expected = String::from("HTTP/1.1 400 BAD REQUEST\r\n\r\n");

        assert_eq!(result, expected);
    }

    #[test]
    fn unit__check_request__elements_with_no_spaces() {
        let request: Vec<String> = vec![String::from("GET/HTTP/1.1")];

        let result = check_request(request);

        let expected = String::from("HTTP/1.1 400 BAD REQUEST\r\n\r\n");

        assert_eq!(result, expected);
    }

    #[test]
    fn unit__check_request__elements_with_four_spaces() {
        let request: Vec<String> = vec![String::from("GET / HTTP/ 1.1")];

        let result = check_request(request);

        let expected = String::from("HTTP/1.1 400 BAD REQUEST\r\n\r\n");

        assert_eq!(result, expected);
    }

    #[test]
    fn unit__check_request__no_http() {
        let request: Vec<String> = vec![String::from("GET / CoAP/ 1.1")];

        let result = check_request(request);

        let expected = String::from("HTTP/1.1 400 BAD REQUEST\r\n\r\n");

        assert_eq!(result, expected);
    }

    #[test]
    fn unit__check_request__invalid_http_version() {
        let request: Vec<String> = vec![String::from("GET / HTTP/3")];

        let result = check_request(request);

        let expected = String::from("HTTP/1.1 505 HTTP VERSION NOT SUPPORTED\r\n\r\n");

        assert_eq!(result, expected);
    }

    #[test]
    fn unit__distribute_with_verb_path__empty_verb() {
        let verb = "";
        let path = "/";

        let result = distribute_with_verb_path(verb, path);

        let expected = String::from("HTTP/1.1 400 BAD REQUEST\r\n\r\n");

        assert_eq!(result, expected);
    }

    #[test]
    fn unit__distribute_with_verb_path__empty_path() {
        let verb = "GET";
        let path = "";

        let result = distribute_with_verb_path(verb, path);

        let expected = String::from("HTTP/1.1 400 BAD REQUEST\r\n\r\n");

        assert_eq!(result, expected);
    }

    #[test]
    fn unit__distribute_with_verb_path__invalid_path() {
        let verb = "GET";
        let path = "/invalid";

        let result = distribute_with_verb_path(verb, path);

        let expected = String::from("HTTP/1.1 404 NOT FOUND\r\n\r\n");

        assert_eq!(result, expected);
    }

    #[test]
    fn unit__distribute_with_verb_path__invalid_http_verb() {
        let verb = "INVALID";
        let path = "/";

        let result = distribute_with_verb_path(verb, path);

        let expected = String::from("HTTP/1.1 404 NOT FOUND\r\n\r\n");

        assert_eq!(result, expected);
    }

    #[test]
    fn integration__check_path__get_method_valid() {
        let request: Vec<String> = vec![String::from("GET / HTTP/1.1")];
        let message = String::from("You did a GET\nHello!\nHi from Rust programming language.");
        let size = message.len();

        let result = check_request(request);

        let expected = format!("HTTP/1.1 200 OK\r\nContent-Length: {size}\r\n\r\n{message}");

        assert_eq!(result, expected);
    }

    #[test]
    fn integration__check_path__post_method_valid() {
        let request: Vec<String> = vec![String::from("POST / HTTP/1.1")];
        let message = String::from("You did a POST\nHello!\nHi from Rust programming language.");
        let size = message.len();

        let result = check_request(request);

        let expected = format!("HTTP/1.1 200 OK\r\nContent-Length: {size}\r\n\r\n{message}");

        assert_eq!(result, expected);
    }

    #[test]
    fn integration__check_path__put_method_valid() {
        let request: Vec<String> = vec![String::from("PUT / HTTP/1.1")];
        let message = String::from("You did a PUT\nHello!\nHi from Rust programming language.");
        let size = message.len();

        let result = check_request(request);

        let expected = format!("HTTP/1.1 200 OK\r\nContent-Length: {size}\r\n\r\n{message}");

        assert_eq!(result, expected);
    }

    #[test]
    fn integration__check_path__delete_method_valid() {
        let request: Vec<String> = vec![String::from("DELETE / HTTP/1.1")];
        let message = String::from("You did a DELETE\nHello!\nHi from Rust programming language.");
        let size = message.len();

        let result = check_request(request);

        let expected = format!("HTTP/1.1 200 OK\r\nContent-Length: {size}\r\n\r\n{message}");

        assert_eq!(result, expected);
    }
}