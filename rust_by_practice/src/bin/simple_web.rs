use rust_by_practice::ThreadPool;
use std::{
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

fn main() {
    const ADDR: &str = "127.0.0.1:7878";
    let listener = TcpListener::bind(ADDR).unwrap();

    println!("Start listening on: {}...", listener.local_addr().unwrap());
    let running = Arc::new(AtomicBool::new(true));

    let running_ctrlc = Arc::clone(&running);
    ctrlc::set_handler(move || {
        running_ctrlc.store(false, Ordering::SeqCst);
        println!("Shutting down server...");
        // trigger loop on listener.incoming() once
        send_programmatic_tcp_connection(ADDR);
    })
    .unwrap();

    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
        if !running.load(Ordering::SeqCst) {
            break;
        }
    }
}

const HELLO_PAGE: &str = "<!DOCTYPE html>
<html lang=\"en\">
  <head>
    <meta charset=\"utf-8\">
    <title>Hello!</title>
  </head>
  <body>
    <h1>Hello!</h1>
    <p>Hi from Rust</p>
  </body>
</html>
";
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let response = if request_line == "GET / HTTP/1.1" {
        format!(
            "{}\r\rContent-Length: {}\r\n\r\n{}",
            "HTTP/1.1 200 OK",
            HELLO_PAGE.len(),
            HELLO_PAGE
        )
    } else {
        "HTTP/1.1 404 NOT FOUND\n\n".to_string()
    };
    stream.write_all(response.as_bytes()).unwrap();
}

// Programmatically sending a connection to the listener
fn send_programmatic_tcp_connection(addr: &str) {
    // Connect to the server
    if let Ok(mut stream) = TcpStream::connect(addr) {
        println!("Sending message to server...");
        stream.write_all(b"OPTIONS / HTTP/1.1\r\n").unwrap();
        stream.flush().unwrap();

        let mut buffer = [0; 512];
        let _ = stream.read(&mut buffer).unwrap();
        println!(
            "Response from server: {}",
            String::from_utf8_lossy(&buffer[..])
        );
    } else {
        eprintln!("Failed to connect to server");
    }
}
