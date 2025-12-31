use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

/* -----------------------------------------
   Handle a single client connection
------------------------------------------ */
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    // Read request
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Request:\n{}", request);

    // Simple HTTP response
    let response = "HTTP/1.1 200 OK\r\n\
                    Content-Type: text/plain\r\n\
                    Content-Length: 13\r\n\
                    \r\n\
                    Hello, Rust!";

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

/* -----------------------------------------
   Main server loop (single-threaded)
------------------------------------------ */
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878")
        .expect("Failed to bind to address");

    println!("Server running on http://127.0.0.1:7878");

    for stream in listener.incoming() {
        let stream = stream.expect("Failed to accept connection");

        // SINGLE-THREADED: blocks until done
        handle_connection(stream);
    }
}
