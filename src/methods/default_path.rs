pub fn get_method () -> String {
    let message = String::from("You did a GET\nHello!\nHi from Rust programming language.");
    let size = message.len();

    let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {size}\r\n\r\n{message}");
    return response;
}

pub fn post_method () -> String {
    let message = String::from("You did a POST\nHello!\nHi from Rust programming language.");
    let size = message.len();

    let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {size}\r\n\r\n{message}");
    return response;
}

pub fn put_method () -> String {
    let message = String::from("You did a PUT\nHello!\nHi from Rust programming language.");
    let size = message.len();

    let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {size}\r\n\r\n{message}");
    return response;
}

pub fn delete_method () -> String {
    let message = String::from("You did a DELETE\nHello!\nHi from Rust programming language.");
    let size = message.len();

    let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {size}\r\n\r\n{message}");
    return response;
}