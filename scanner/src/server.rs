use std::net::{TcpListener,TcpStream};
use std::io::*;

pub fn server() {
  let site = TcpListener::bind("127.0.0.1:8080").unwrap();
  let mut buf = [0; 1024];
  let mut read_client = 0;
  for stream in site.incoming() {
    let mut stream: TcpStream = stream.unwrap();
    println!("SERVER_CONNECT: {:?}", stream);
    read_client = stream.read(&mut buf).expect("error");
    println!("bytes sent {}", read_client);
    break
}
  stdout().write(&buf[0..read_client]).unwrap();
  

}