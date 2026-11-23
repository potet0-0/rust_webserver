use std::fs;
use std::io::{Write, BufReader, BufRead};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = match TcpListener::bind("127.0.0.1:7878") {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Failed to bind to 127.0.0.1:7878: {}", e);
            std::process::exit(1);
        }
    };
    println!("Listening on http://127.0.0.1:7878");

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            handle_connection(stream);
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut reader = BufReader::new(&stream);
    let mut request_line = String::new();
    if reader.read_line(&mut request_line).is_err() {
        return;
    }

    let parts: Vec<&str> = request_line.trim().split_whitespace().collect();
    if parts.len() < 2 {
        return;
    }

    let method = parts[0];
    let path = parts[1];

    // Drain headers
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line).is_err() || line.trim().is_empty() {
            break;
        }
    }

    let (status, content_type, body) = if method == "GET" && path == "/" {
        let html = fs::read_to_string("static/index.html").unwrap_or_else(|_| "<h1>Hello</h1>".to_string());
        ("200 OK", "text/html", html)
    } else if method == "GET" && path == "/health" {
        ("200 OK", "text/plain", "ok".to_string())

    } else if method == "GET" && path == "/main" {
        let html = fs::read_to_string("static/main.html").unwrap_or_else(|_| "main".to_string());
        ("200 OK", "text/html", html)
    }
    else {
        let html = fs::read_to_string("static/404.html").unwrap_or_else(|_| "not found".to_string());
        ("404 Not Found", "text/html", html)
    };

    let response = format!(
        "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        status,
        content_type,
        body.len(),
        body
    );

    let _ = stream.write_all(response.as_bytes());
    let _ = stream.flush();
}
