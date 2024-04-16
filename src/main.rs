use std::{fs::read_to_string, io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}};

use apple::ThreadPool;

fn main() {
    let listner = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listner.incoming().take(2) {
        let stream = stream.unwrap();
        pool.execute(||{
            handle_connection(stream)
        }
        )
    }
    println!("server shut down")
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, html_path) = if http_request == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let content = read_to_string(html_path).unwrap();
    let response = format!("{status_line}\r\n\r\n\r\n{content}");

    stream.write_all(response.as_bytes()).unwrap();
}


