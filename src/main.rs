// rust
use std::net::TcpListener;


fn main () -> std::io::Result<()> {
    // let listener = TcpListener::bind("localhost:8080").unwrap_or_else(|error| {
    //     println!("Error: {:?}", error);
    // });

    let listener = TcpListener::bind("localhost:8080")?;
    println!("Listening: {:?}", listener.local_addr()?);

    for data_incoming in listener.incoming() {

    }
    Ok(())
}