// Imports
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap_or_else(|err| {
        eprintln!("Error binding to port 8000: {}", err);
        std::process::exit(1);
    });
    println!("Server listening on port 8000");

    for stream in listener.incoming() {
        println!("Connection established!");
        let stream = stream.unwrap_or_else(|err| {
            eprintln!("Error establishing connection: {}", err);
            std::process::exit(1);
        });
        handle_connection(stream);
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    if let Err(e) = stream.read(&mut buffer) {
        eprintln!("Failed to read from connection: {}", e);
        return;
    }

    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Request: {}", request);

    let (status_line, filename, content_type) = if buffer.starts_with(b"GET / HTTP/1.1\r\n") {
        ("HTTP/1.1 200 OK", "index.html", "text/html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html", "text/html")
    };

    let contents = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to read file {}: {}", filename, e);
            return; // Exit the function gracefully if the file cannot be read
        }
    };
    
    let response = format!(
        "{}\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        content_type,
        contents
    );

    // Safely attempt to write to the stream
    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("Failed to write response to connection: {}", e);
    }

    // Flush the stream (finalize the write operation)
    if let Err(e) = stream.flush() {
        eprintln!("Failed to flush connection: {}", e);
    }
}
