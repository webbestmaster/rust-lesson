use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Не удалось запустить сервер");

    println!("Сервер запущен на http://127.0.0.1:7878");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer);

    let path = request
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .unwrap_or("/");
    
    let (status, body) = match path {
        "/" => ("HTTP/1.1 200 OK", "Главная страница"),
        "/hello" => ("HTTP/1.1 200 OK", "Привет, Rust!"),
        "/about" => ("HTTP/1.1 200 OK", "О сервере"),
        _ => ("HTTP/1.1 404 NOT FOUND", "Страница не найдена"),
    };

    let response = format!(
        "{status}\r\n\
        Content-Type: text/plain; charset=utf-8\r\n\
        Content-Length: {}\r\n\
        \r\n\
        {}",
        body.as_bytes().len(),
        body
    );

    stream.write_all(response.as_bytes()).unwrap();
}
