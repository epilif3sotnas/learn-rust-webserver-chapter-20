// rust
use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, prelude::*};

// internal
mod methods;

use crate::methods::default_path::{
    get_method,
    post_method,
    put_method,
    delete_method
};

const HTTP_VERSIONS: [&'static str; 3] = ["1", "1.1", "2"];
const HTTP_PROTOCOL: &'static str = "HTTP";


fn main () -> std::io::Result<()> {
    let listener = TcpListener::bind("localhost:8080")?;
    println!("Listening: {:?}", listener.local_addr()?);

    for stream in listener.incoming() {
        println!("Stream: {:?}", stream);
        println!("Something connected");

        handle_connection(stream?);
    }
    Ok(())
}

fn handle_connection (mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let http_request: Vec<_> = buf_reader
                            .lines()
                            .map(|line| line.unwrap())
                            .take_while(|line| !line.is_empty())
                            .collect();

    println!("Got HTTP request: {:?}", http_request);

    let response = check_request(http_request);

    stream.write_all(response.as_bytes()).unwrap_or_else(|err| {
        println!("Error: {:?}", err);
    });
}

fn check_request (request: Vec<String>) -> String {
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

fn distribute_with_verb_path (verb: &str, path: &str) -> String {
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
        _ => return String::from("HTTP/1.1 404 NOT FOUND\r\n\r\n")
    }
}