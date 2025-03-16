use std::net::TcpStream;

fn main() {
  let client_tcp = TcpStream::connect("127.0.0.1:8080");
}