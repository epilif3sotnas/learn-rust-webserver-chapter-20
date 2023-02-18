// rust
use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, prelude::*};


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
}