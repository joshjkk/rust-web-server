use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::fs;

pub struct Server {
    pub ip_addr: String
}

impl Server {
    pub fn new(ip: &str) -> Server {
        Server {
            ip_addr: ip.to_string()
        }
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        let mut buffer = [0; 2048];

        stream.read(&mut buffer).unwrap();

        let get = b"GET / HTTP/1.1\r\n";

        let (status_line, filename) = if buffer.starts_with(get) {
            ("HTTP/1.1 200 OK", "index.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };

        let contents = fs::read_to_string(
            format!("templates/{}", filename)).unwrap();

        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}", 
            status_line, contents.len(), contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    pub fn start(&self) {
        let listener = TcpListener::bind(&self.ip_addr).unwrap();

        println!("\n==== [START] Waiting for connection on {} ===\n", self.ip_addr);

        for stream in listener.incoming() {
            self.handle_connection(stream.unwrap());
        }
    }
}
