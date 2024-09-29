// Import the TCP listener
use std::{
    fmt::Result,
    fs,
    io::{prelude::*, BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use webserver::ThreadPool;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();

    // To make the code more fast I would like to create multpile threads
    let pool = ThreadPool::new(4);

    // fetch the stream
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    // Read the stream into a bufreader just a way to store text
    let buf_reader = BufReader::new(&mut stream);
    let first_line = buf_reader.lines().next().unwrap().unwrap();

    // lets pass some values into a tuple
    let (status_line, filename) = if first_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    // Lets serve the webpage back
    let content = fs::read_to_string(filename).unwrap();
    let headers = content.len();

    let response = format!("{status_line}Content-Length: {headers}\r\n\r\n{content}");
    stream.write_all(response.as_bytes()).unwrap();
}
