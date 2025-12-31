use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

/* -----------------------------------------
   Handle a client connection
------------------------------------------ */
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Request:\n{}", request);

    let response = "HTTP/1.1 200 OK\r\n\
                    Content-Type: text/plain\r\n\
                    Content-Length: 19\r\n\
                    \r\n\
                    Hello from threads!";

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

/* -----------------------------------------
   Multi-threaded server
------------------------------------------ */
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878")
        .expect("Failed to bind address");

    println!("Multi-threaded server running on http://127.0.0.1:7878");

    for stream in listener.incoming() {
        let stream = stream.expect("Failed to accept connection");

        // Spawn a new OS thread per connection
        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}
