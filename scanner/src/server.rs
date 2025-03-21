use std::net::{TcpListener,TcpStream};
use std::io::*;

pub fn server() {
  let site = TcpListener::bind("127.0.0.1:8080").unwrap();
  let mut buf = [0; 512];
  let mut read_client = 0;
  for stream in site.incoming() {
    let mut stream: TcpStream = stream.unwrap();
    println!("SERVER_CONNECT: {:?}", stream);
    
      while read_client > 0 {
         
          read_client = Result::expect(stream.read(&mut buf), "Error");
          println!("bytes sent {}", read_client);
  }
}
  stdout().write(&buf[0..read_client]).unwrap();
  

}